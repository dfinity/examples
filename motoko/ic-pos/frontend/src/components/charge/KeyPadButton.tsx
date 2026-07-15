import type { ReactElement } from "react";
import { Button } from "../ui/button";

type KeyPadButtonProps = {
  label: string | ReactElement;
  onClick: () => void;
};

export function KeyPadButton({ label, onClick }: KeyPadButtonProps) {
  return (
    <Button onClick={onClick} variant={"outline"}>
      {label}
    </Button>
  );
}
