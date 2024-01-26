import { Delete } from "lucide-react";
import { Key } from "../types/key.type";
import { KeyPadButton } from "./KeyPadButton";

type KeyPadProps = {
  onKey: (key: Key) => void;
};

export function KeyPad({ onKey }: KeyPadProps) {
  return (
    <div className="grid w-full grid-cols-3 gap-5 p-5">
      {["1", "2", "3", "4", "5", "6", "7", "8", "9"].map((num) => (
        <KeyPadButton key={num} label={num} onClick={() => onKey(num as Key)} />
      ))}
      <KeyPadButton label="." onClick={() => onKey("decimal")} />
      <KeyPadButton label="0" onClick={() => onKey("0")} />
      <KeyPadButton label={<Delete />} onClick={() => onKey("backspace")} />
    </div>
  );
}
