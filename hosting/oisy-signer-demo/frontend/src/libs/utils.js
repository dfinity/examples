export const cn = (...classes) => {
  return classes.filter(Boolean).join(' ');
};

export const toBaseUnits = (amount, decimals) => {
  return BigInt(Math.round(amount * 10 ** decimals));
};

export const toMainUnit = (amount, decimals) => {
  return Number(amount) / 10 ** decimals;
};
