// A `hasOwnProperty` that produces evidence for the typechecker
export function hasOwnProperty<
  X extends Record<string, unknown>,
  Y extends PropertyKey
>(obj: X, prop: Y): obj is X & Record<Y, unknown> {
  return Object.prototype.hasOwnProperty.call(obj, prop);
}

// Turns an 'unknown' into a string, if possible, otherwise use the default
// `def` parameter.
export function unknownToString(obj: unknown, def: string): string {
  // Only booleans, numbers and strings _may_ not be objects, so first we try
  // Object's toString, and if not we go through the remaining types.
  if (obj instanceof Object) {
    return obj.toString();
  } else if (typeof obj === "string") {
    return obj;
  } else if (typeof obj === "number") {
    return obj.toString();
  } else if (typeof obj === "boolean") {
    return obj.toString();
  }

  // Only "null" and "undefined" do not have 'toString', though typescript
  // doesn't know that.
  return def;
}

// Returns true if we're in Safari or iOS (although technically iOS only has
// Safari)
export function iOSOrSafari(): boolean {
  // List of values of navigator.userAgent, navigator.platform and
  // navigator.userAgentData by device so far (note: navigator.platform is
  // deprecated but navigator.userAgentdata is not implemented in many
  // browsers):
  //
  // iPhone 12 Mini, iOS 15.0.2
  //
  // Safari
  // navigator.userAgentData: undefined
  // navigator.platform: "iPhone"
  // navigator.userAgent: "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"
  //
  //
  // MacBook Pro Intel, MacOS Big Sur 11.6
  //
  // Safari
  // navigator.userAgentData: undefined
  // navigator.platform: "MacIntel"
  // navigator.userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15"
  //
  // Chrome
  // navigator.userAgentData.plaftorm: "macOS"
  // navigator.platform: "MacIntel"
  // navigator.userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36"
  //
  // Firefox
  // navigator.userAgentData: undefined
  // navigator.platform: "MacIntel"
  // navigator.userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:93.0) Gecko/20100101 Firefox/93.0"
  //
  //
  // MacBook Air M1, MacOS Big Sur 11.6
  //
  // Safari
  // navigator.userAgentData: undefined
  // navigator.platform: "MacIntel" // yes, I double checked
  // navigator.userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Safari/605.1.15"
  //
  // Firefox
  // navigator.userAgentData: undefined
  // navigator.platform: "MacIntel" // yes, I double checked
  //
  // iPad Pro, iPadOS 15.0.2
  //
  // Safari
  // navigator.userAgentData: undefined
  // navigator.platform: "iPad"
  // navigator.userAgent: "Mozilla/5.0 (iPad; CPU OS 15_0_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"

  // For details, see https://stackoverflow.com/a/23522755/2716377
  return /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
}
