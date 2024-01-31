import { Button } from "../../../components/ui/button";
import { Link } from "@tanstack/router";

export default function ReceiveButton() {
  return (
    <Link to="/charge">
      <Button size={"lg"} className="w-56">
        Charge
      </Button>
    </Link>
  );
}
