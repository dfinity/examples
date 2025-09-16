use crate::treasury_manager::{AuditTrail, Operation, Step, TreasuryManagerOperation};

pub const MAX_REPLY_SIZE_BYTES: usize = 1_024;

pub fn serialize_audit_trail(
    audit_trail: &AuditTrail,
    make_pretty: bool,
) -> Result<String, String> {
    let result = if make_pretty {
        serde_json::to_string_pretty(&audit_trail.transactions)
    } else {
        serde_json::to_string(&audit_trail.transactions)
    };
    result.map_err(|err| format!("{err:?}"))
}

#[must_use]
#[derive(Debug)]
pub struct OperationContext {
    pub operation: Operation,

    /// None indicates that there were no calls yet.
    index: Option<usize>,
}

impl OperationContext {
    pub fn new(operation: Operation) -> Self {
        Self {
            operation,
            index: None,
        }
    }

    /// Should be used for operations that are definitely not the final operation
    /// of the current operation.
    pub fn next_operation(&mut self) -> TreasuryManagerOperation {
        let operation = self.operation;

        let index = self
            .index
            // If index is available, increment it by 1.
            .map(|index| index.saturating_add(1))
            // Otherwise, start from 0.
            .unwrap_or_default();

        self.index = Some(index);

        let step = Step {
            index,
            is_final: false,
        };

        TreasuryManagerOperation { operation, step }
    }
}

/// TAKEN FROM: ic/rs/nervous_system/string/src/lib.rs
///
/// Returns a possibly modified version of `s` that fits within the specified bounds (in terms of
/// the number of UTF-8 characters).
///
/// More precisely, middle characters are removed such that the return value has at most `max_len`
/// characters.
///
/// This is analogous clamp method on numeric types in that this makes the value bounded.
pub fn clamp_string_len(s: &str, max_len: usize) -> String {
    // Collect into a vector so that we can safely index the input.
    let chars: Vec<_> = s.chars().collect();
    if max_len <= 3 {
        return chars.into_iter().take(max_len).collect();
    }

    if chars.len() <= max_len {
        return s.to_string();
    }

    let ellipsis = "...";
    let content_len = max_len - ellipsis.len();
    let tail_len = content_len / 2;
    let head_len = content_len - tail_len;
    let tail_begin = chars.len() - tail_len;

    format!(
        "{}{}{}",
        chars[..head_len].iter().collect::<String>(),
        ellipsis,
        chars[tail_begin..].iter().collect::<String>(),
    )
}

fn utf8_to_ascii_lossy(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii() { c } else { '?' })
        .collect()
}

pub fn serialize_reply<R>(reply: &R) -> String
where
    R: serde::Serialize,
{
    let Ok(json_str) = serde_json::to_string(reply) else {
        return "<Failed serializing reply>".to_string();
    };

    let json_str = utf8_to_ascii_lossy(&json_str);

    clamp_string_len(&json_str, MAX_REPLY_SIZE_BYTES)
}
