// Convert a Motoko optional to a JavaScript object.
export function fromOptional(optional) {
  return optional.length > 0 ? optional[0] : null;
}

// Convert a JavaScript object to a Motoko optional.
export function toOptional(object) {
  return object ? [object] : [];
}
