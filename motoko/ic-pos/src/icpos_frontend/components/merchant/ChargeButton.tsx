import { Link } from "@tanstack/react-router";
import { Button } from "../ui/button";

export default function ReceiveButton() {
  return (
    <Link to="/charge">
      <Button size={"lg"} className="w-56">
        Charge
      </Button>
    </Link>
  );
}
