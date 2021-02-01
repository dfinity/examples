// Convert a Motoko list to a JavaScript array.
export function fromList(list) {
  if (list.length == 0) {
    return [];
  } else {
    const tuple = list[0];
    const array = fromList(tuple[1]);
    array.unshift(tuple[0]);
    return array;
  }
}

// Convert a JavaScript array to a Motoko list.
export function toList(array) {
  return array.reduceRight((accum, x) => {
    return [[x, accum]];
  }, []);
}

// Convert a Motoko optional to a JavaScript object.
export function fromOptional(optional) {
  return optional.length > 0 ? optional[0] : null;
}

// Convert a JavaScript object to a Motoko optional.
export function toOptional(object) {
  return object ? [object] : [];
}
