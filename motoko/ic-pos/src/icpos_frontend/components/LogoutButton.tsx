import { LogOut } from "lucide-react";
import { Button } from "./ui/button";
import { useAuth } from "@/lib/auth";
import { useNavigate } from "@tanstack/react-router";
import { router } from "@/main";

export default function LogoutButton() {
  const { clear } = useAuth();
  const navigate = useNavigate();

  const logout = () => {
    void clear();
    setTimeout(() => {
      router.invalidate();
      navigate({ to: '/' });
    }, 100)
  }

  return <Button variant="ghost" size="icon" onClick={logout} className="hover:text-black">
    <LogOut
      className="w-4 h-4"
    />
  </Button>
}

