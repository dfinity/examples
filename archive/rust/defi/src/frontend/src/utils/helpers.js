import { sha224 } from "js-sha256";
import { Buffer } from "buffer";
import crc from "crc";


export const toHexString = (byteArray) => {
    return Array.from(byteArray, function(byte) {
        return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('').toUpperCase();
};

export const hexToBytes = (hex) => {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
};


export const uint8ArrayToBigInt = (array) => {
  const view = new DataView(array.buffer, array.byteOffset, array.byteLength);
  if (typeof view.getBigUint64 === "function") {
    return view.getBigUint64(0);
  } else {
    const high = BigInt(view.getUint32(0));
    const low = BigInt(view.getUint32(4));

    return (high << BigInt(32)) + low;
  }
};

const TWO_TO_THE_32 = BigInt(1) << BigInt(32);
export const bigIntToUint8Array = (value) => {
  const array = new Uint8Array(8);
  const view = new DataView(array.buffer, array.byteOffset, array.byteLength);
  if (typeof view.setBigUint64 === "function") {
    view.setBigUint64(0, value);
  } else {
    view.setUint32(0, Number(value >> BigInt(32)));
    view.setUint32(4, Number(value % TWO_TO_THE_32));
  }

  return array;
};

export const arrayBufferToArrayOfNumber = (
  buffer
)=> {
  const typedArray = new Uint8Array(buffer);
  return Array.from(typedArray);
};

export const arrayOfNumberToUint8Array = (
  numbers
) => {
  return new Uint8Array(numbers);
};

export const arrayOfNumberToArrayBuffer = (
  numbers
) => {
  return arrayOfNumberToUint8Array(numbers).buffer;
};

export const arrayBufferToNumber = (buffer) => {
  const view = new DataView(buffer);
  return view.getUint32(view.byteLength - 4);
};

export const numberToArrayBuffer = (
  value,
  byteLength
) => {
  const buffer = new ArrayBuffer(byteLength);
  new DataView(buffer).setUint32(byteLength - 4, value);
  return buffer;
};

export const asciiStringToByteArray = (text)=> {
  return Array.from(text).map((c) => c.charCodeAt(0));
};

export const toSubAccountId = (subAccount) => {
  const bytes = arrayOfNumberToArrayBuffer(subAccount);
  return arrayBufferToNumber(bytes);
};

export const accountIdentifierToBytes = (
  accountIdentifier
) => {
  return Uint8Array.from(Buffer.from(accountIdentifier, "hex")).subarray(4);
};

export const accountIdentifierFromBytes = (
  accountIdentifier
) => {
  return Buffer.from(accountIdentifier).toString("hex");
};

export const principalToAccountDefaultIdentifier = (
  principal,
) => {
  // Hash (sha224) the principal, the subAccount and some padding
  const padding = asciiStringToByteArray("\x0Aaccount-id");

  const shaObj = sha224.create();
  shaObj.update([
    ...padding,
    ...principal.toUint8Array(),
    ...(Array(32).fill(0)),
  ]);
  const hash = new Uint8Array(shaObj.array());

  // Prepend the checksum of the hash and convert to a hex string
  const checksum = calculateCrc32(hash);
  const bytes = new Uint8Array([...checksum, ...hash]);
  return toHexString(bytes);
};

export const principalToSubAccount = (principal) => {
  const bytes = principal.toUint8Array();
  const subAccount = new Uint8Array(32);
  subAccount[0] = bytes.length;
  subAccount.set(bytes, 1);
  return subAccount;
};


// 4 bytes
export const calculateCrc32 = (bytes) => {
  const checksumArrayBuf = new ArrayBuffer(4);
  const view = new DataView(checksumArrayBuf);
  view.setUint32(0, crc.crc32(Buffer.from(bytes)), false);
  return Buffer.from(checksumArrayBuf);
};