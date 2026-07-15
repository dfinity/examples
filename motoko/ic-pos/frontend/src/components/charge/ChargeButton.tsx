import { Link } from "@tanstack/react-router";
import { Button } from "../ui/button";

type ChargeButtonProps = {
  amount: string;
};
export default function ChargeButton({ amount }: ChargeButtonProps) {
  return (
    <Link to={`/receive`} search={{ amount }} className="w-full px-5">
      <Button size={"lg"} className="w-full">
        Charge
      </Button>
    </Link>
  );
}
