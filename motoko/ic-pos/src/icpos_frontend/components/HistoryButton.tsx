import { Link } from "@tanstack/react-router";
import { Button } from "./ui/button";
import { Inbox } from "lucide-react";

export default function HistoryButton() {
  return (
    <Link
      to="/history"
      className="print:hidden"
    >
      <Button size={"lg"} className="w-56">
        <Inbox className="w-4 h-4 mr-2" />
        History
      </Button>
    </Link>
  );
}
