export const getUserNumber = (): bigint | undefined => {
  const userNumber = localStorage.getItem("userNumber");
  return userNumber !== null ? BigInt(userNumber) : undefined;
};

export const setUserNumber = (userNumber: bigint | undefined): void => {
  if (userNumber !== undefined) {
    localStorage.setItem("userNumber", userNumber.toString());
  } else {
    localStorage.removeItem("userNumber");
  }
};

// BigInt parses various things we do not want to allow, like:
// - BigInt(whitespace) == 0
// - Hex/Octal formatted numbers
// - Scientific notation
// So we check that the user has entered a sequence of digits only,
// before attempting to parse
export const parseUserNumber = (s: string): bigint | null => {
  if (/^\d+$/.test(s)) {
    try {
      return BigInt(s);
    } catch (err) {
      return null;
    }
  } else {
    return null;
  }
};
