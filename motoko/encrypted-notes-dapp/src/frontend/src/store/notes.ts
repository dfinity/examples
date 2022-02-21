import { writable } from 'svelte/store';
import type { BackendActor } from '../lib/actor';
import type { EncryptedNote } from '../lib/backend';
import type { CryptoService } from '../lib/crypto';
import { deserialize, NoteModel, serialize } from '../lib/note';
import { auth } from './auth';
import { showError } from './notifications';

export const notesStore = writable<
  | {
      state: 'uninitialized';
    }
  | {
      state: 'loading';
    }
  | {
      state: 'loaded';
      list: NoteModel[];
    }
  | {
      state: 'error';
    }
>({ state: 'uninitialized' });

let notePollerHandle: ReturnType<typeof setInterval> | null;

async function decryptNotes(
  notes: EncryptedNote[],
  cryptoService: CryptoService
): Promise<NoteModel[]> {
  return await Promise.all(
    notes.map((encryptedNote) => deserialize(encryptedNote, cryptoService))
  );
}

function updateNotes(notes: NoteModel[]) {
  notesStore.set({
    state: 'loaded',
    list: notes,
  });
}

export async function refreshNotes(
  actor: BackendActor,
  cryptoService: CryptoService
) {
  const encryptedNotes = await actor.get_notes();

  // did we get logged out?
  if (!cryptoService.isInitialized()) return;

  const notes = await decryptNotes(encryptedNotes, cryptoService);
  updateNotes(notes);
}

export async function addNote(
  note: NoteModel,
  actor: BackendActor,
  crypto: CryptoService
) {
  const encryptedNote = (await serialize(note, crypto)).encrypted_text;
  await actor.add_note(encryptedNote);
}
export async function updateNote(
  note: NoteModel,
  actor: BackendActor,
  crypto: CryptoService
) {
  const encryptedNote = await serialize(note, crypto);
  await actor.update_note(encryptedNote);
}

auth.subscribe(async ($auth) => {
  if ($auth.state === 'initialized') {
    if (notePollerHandle !== null) {
      clearInterval(notePollerHandle);
      notePollerHandle = null;
    }

    notesStore.set({
      state: 'loading',
    });
    try {
      await refreshNotes($auth.actor, $auth.crypto).catch((e) =>
        showError(e, 'Could not poll notes.')
      );

      notePollerHandle = setInterval(async () => {
        await refreshNotes($auth.actor, $auth.crypto).catch((e) =>
          showError(e, 'Could not poll notes.')
        );
      }, 3000);
    } catch {
      notesStore.set({
        state: 'error',
      });
    }
  } else if ($auth.state === 'anonymous' && notePollerHandle !== null) {
    clearInterval(notePollerHandle);
    notePollerHandle = null;
    notesStore.set({
      state: 'uninitialized',
    });
  }
});
