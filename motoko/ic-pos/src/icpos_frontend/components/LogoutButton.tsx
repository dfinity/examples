import { LogOut } from "lucide-react";
import { Button } from "./ui/button";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useNavigate } from "@tanstack/react-router";
import { router } from "@/main";

export default function LogoutButton() {
  const { clear } = useInternetIdentity();
  const navigate = useNavigate();

  const logout = () => {
    clear();
    setTimeout(() => {
      router.invalidate();
      navigate({ to: '/' });
    }, 500)
  }

  return <Button variant="ghost" size="icon" onClick={logout}>
    <LogOut
      className="w-4 h-4"
    />
  </Button>
}

