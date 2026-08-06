#!/usr/bin/env bash
# sync-ic-skills.sh — mirror the latest Internet Computer skills into .claude/skills/
#
# Differential sync: fetches the discovery index once and re-downloads only the
# skills whose published `hash` changed (or are new). Skills already at the current
# hash are skipped entirely — no per-file downloads. Prints a one-line summary only
# when something actually changed.
#
# Idempotent and offline-safe. Only skills this script installed are ever pruned,
# so your own local skills are never touched.
set -euo pipefail

BASE="https://skills.internetcomputer.org/.well-known/skills"
INDEX_URL="$BASE/index.json"
DEST=".claude/skills"
MANIFEST="$DEST/.ic-managed.json"   # { "<skill>": "<hash>" } of skills this script manages

mkdir -p "$DEST"

# --- Temp files. NEW_MANIFEST is built up as we go, then swapped in atomically.
#     STAGING holds the skill dir currently being downloaded, so the trap can
#     remove a half-written skill if the run is interrupted. ---
TMP_INDEX="$(mktemp)"
NEW_MANIFEST="$(mktemp)"
STAGING=""
trap 'rm -f "$TMP_INDEX" "$NEW_MANIFEST"; [ -n "$STAGING" ] && rm -rf "$STAGING"' EXIT

# Remove any staging dirs left by a previously interrupted run — an in-progress
# download is always safe to discard. (.old-* backups are handled by the recovery
# step below, which never deletes one that is still the only copy of a skill.)
rm -rf "${DEST:?}"/.staging-* 2>/dev/null || true

# --- Path-safety guards. `name` and `f` come from the remote index and flow into
#     rm -rf / mv / file writes, so reject anything that could escape $DEST. ---
is_safe_name() {   # a flat skill slug: non-empty, no slash, no ".."
  case "$1" in
    ""|.|..|*/*|*..*) return 1 ;;
    *) return 0 ;;
  esac
}
is_safe_relpath() {  # a file path within a skill: subdirs ok, but not absolute or ".."
  case "$1" in
    ""|/*|*..*) return 1 ;;
    *) return 0 ;;
  esac
}

# --- Recover from a run interrupted mid-swap. A `.old-<name>.<pid>` dir is the
#     previous good copy of <name>, moved aside just before its swap. If that swap
#     never finished (the skill dir is now missing), restore it; otherwise it is
#     stale and safe to drop. This runs BEFORE the index fetch, so an interrupted
#     skill is restored even on an offline run — keeping the cached copy available. ---
for backup in "$DEST"/.old-*; do
  [ -e "$backup" ] || continue                 # unmatched glob stays literal — skip
  bname="$(basename "$backup")"; bname="${bname#.old-}"; bname="${bname%.*}"
  if is_safe_name "$bname" && [ ! -e "$DEST/$bname" ]; then
    mv "$backup" "$DEST/$bname"
    echo "[autosync-ic-skills] recovered '$bname' from an interrupted sync" >&2
  else
    rm -rf "$backup"
  fi
done

# --- Fetch the index. On any network failure, keep cached skills and exit cleanly. ---
if ! curl -fsSL --max-time 20 "$INDEX_URL" -o "$TMP_INDEX"; then
  echo "[autosync-ic-skills] could not reach $INDEX_URL — keeping cached skills" >&2
  exit 0
fi

# --- jq is required to parse the index. If absent, warn and exit without failing. ---
if ! command -v jq >/dev/null 2>&1; then
  echo "[autosync-ic-skills] 'jq' not found — install jq to enable IC skill sync" >&2
  exit 0
fi

# --- Previously-managed skill names. Supports the legacy manifest format
#     (a bare array of names, no hashes) as well as the current object form. ---
managed_names() {
  [ -f "$MANIFEST" ] || return 0
  jq -r 'if type == "object" then keys[] elif type == "array" then .[] else empty end' \
    "$MANIFEST" 2>/dev/null || true
}

# --- Stored hash for a skill, or empty if unknown (new skill, or legacy manifest). ---
stored_hash() {
  [ -f "$MANIFEST" ] || return 0
  jq -r --arg n "$1" 'if type == "object" then (.[$n] // "") else "" end' \
    "$MANIFEST" 2>/dev/null || true
}

# --- Append a name->hash pair to the new manifest being built. ---
record() {
  local tmp; tmp="$(mktemp)"
  jq --arg n "$1" --arg h "$2" '.[$n] = $h' "$NEW_MANIFEST" > "$tmp" && mv "$tmp" "$NEW_MANIFEST"
}

NEW_NAMES="$(jq -r '.skills[].name' "$TMP_INDEX")"
MANAGED="$(managed_names)"
echo '{}' > "$NEW_MANIFEST"

# --- Prune: drop previously-managed skills that are no longer in the index. ---
removed=0
while IFS= read -r old; do
  [ -n "$old" ] || continue
  is_safe_name "$old" || { echo "[autosync-ic-skills] skipping unsafe managed name: $old" >&2; continue; }
  if ! grep -qxF "$old" <<<"$NEW_NAMES"; then
    rm -rf "${DEST:?}/$old"
    removed=$((removed + 1))
    echo "[autosync-ic-skills] removed: $old" >&2
  fi
done <<<"$MANAGED"

# --- Sync: download only skills whose hash changed (new / hashless always download). ---
added=0; updated=0; unchanged=0
while IFS= read -r entry; do
  name="$(jq -r '.name' <<<"$entry")"
  [ -n "$name" ] && [ "$name" != "null" ] || continue
  is_safe_name "$name" || { echo "[autosync-ic-skills] skipping skill with unsafe name: $name" >&2; continue; }
  new_hash="$(jq -r '.hash // ""' <<<"$entry")"
  old_hash="$(stored_hash "$name")"

  # Skip when the hash is known, unchanged, and the files are already on disk.
  if [ -n "$new_hash" ] && [ "$new_hash" = "$old_hash" ] && [ -d "$DEST/$name" ]; then
    unchanged=$((unchanged + 1))
    record "$name" "$new_hash"
    continue
  fi

  # Otherwise download this skill into a fresh staging dir, then swap it in
  # atomically. A clean staging dir means an intra-skill file rename or removal
  # leaves no orphaned files behind, and a mid-download failure keeps the existing
  # copy intact — the swap happens only after every file downloaded successfully.
  ok=1
  STAGING="$(mktemp -d "${DEST}/.staging-${name}.XXXXXX")"
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    if ! is_safe_relpath "$f"; then
      echo "[autosync-ic-skills] warning: unsafe file path in $name: $f — skipping skill" >&2
      ok=0
      break
    fi
    mkdir -p "$(dirname "$STAGING/$f")"   # files may live in subdirs (e.g. scripts/)
    if ! curl -fsSL --max-time 20 "$BASE/$name/$f" -o "$STAGING/$f"; then
      echo "[autosync-ic-skills] warning: failed to fetch $name/$f" >&2
      ok=0
      break
    fi
  done < <(jq -r '.files[]?' <<<"$entry")

  if [ "$ok" -eq 1 ]; then
    # Swap in the fresh copy. Move any existing dir aside first, move the new one
    # into place, and only then drop the old copy — so a failed swap restores the
    # existing copy intact, while files removed or renamed upstream don't survive.
    backup=""
    if [ -e "$DEST/$name" ]; then
      backup="${DEST}/.old-${name}.$$"
      rm -rf "$backup"
      mv "$DEST/$name" "$backup"
    fi
    if mv "$STAGING" "$DEST/$name"; then
      STAGING=""
      [ -n "$backup" ] && rm -rf "$backup"
      # Record the new hash so the next run can skip this skill. A hashless server
      # records an empty hash, which never equals new_hash -> always re-downloads.
      record "$name" "$new_hash"
      if grep -qxF "$name" <<<"$MANAGED"; then
        updated=$((updated + 1))
      else
        added=$((added + 1))
      fi
    else
      # Swap failed: restore any existing copy and retry on the next run.
      echo "[autosync-ic-skills] warning: failed to install $name — kept any existing copy; will retry next run" >&2
      [ -n "$backup" ] && mv "$backup" "$DEST/$name"
      rm -rf "$STAGING"
      STAGING=""
      record "$name" "$old_hash"
    fi
  else
    # Download incomplete: discard the staging dir, keep the existing skill dir
    # untouched, and keep the old hash so the next run retries this skill.
    rm -rf "$STAGING"
    STAGING=""
    record "$name" "$old_hash"
  fi
done < <(jq -c '.skills[]' "$TMP_INDEX")

# --- Swap in the updated manifest. ---
mv "$NEW_MANIFEST" "$MANIFEST"

# --- Report only when something changed; stay silent on a no-op sync. ---
# SessionStart hook stdout/stderr is NOT shown in the Claude Code UI — only JSON
# fields are surfaced. We emit a single JSON object on stdout:
#   - systemMessage    -> rendered to the USER as a visible system notice
#   - additionalContext -> injected into Claude's context so it can mention it too
if [ $((added + updated + removed)) -gt 0 ]; then
  summary="[autosync-ic-skills] ${added} added, ${updated} updated, ${removed} removed (${unchanged} unchanged) in $DEST"
  jq -n --arg msg "$summary" '{
    systemMessage: $msg,
    hookSpecificOutput: {
      reloadSkills: true,
      hookEventName: "SessionStart",
      additionalContext: $msg
    }
  }'
fi
