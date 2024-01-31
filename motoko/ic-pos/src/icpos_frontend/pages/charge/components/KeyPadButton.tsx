import { Button } from "../../../components/ui/button";

type KeyPadButtonProps = {
  label: string | JSX.Element;
  onClick: () => void;
};

export function KeyPadButton({ label, onClick }: KeyPadButtonProps) {
  return (
    <Button onClick={onClick} variant={"outline"}>
      {label}
    </Button>
  );
}
