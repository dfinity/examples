import { Link } from "@tanstack/react-router";
import { Button } from "./ui/button";
import { Inbox } from "lucide-react";

type HistoryButtonProps = {
  principal: string;
};

export default function HistoryButton({ principal }: HistoryButtonProps) {
  return (
    <Link
      to="/history"
      search={{
        principal,
      }}
      className="print:hidden"
    >
      <Button size={"lg"} className="w-56">
        <Inbox className="w-4 h-4 mr-2" />
        History
      </Button>
    </Link>
  );
}
