import DfinityLogo from "../../assets/dfinity-logo.png";
import { useAuth } from "@/lib/auth";
import { Button } from "../ui/button";
import { CheckCircle, CircleX, LoaderCircle } from "lucide-react";
import { useState, type ReactElement } from "react";

type LoginStatus = "idle" | "logging-in" | "success" | "error";

export default function LoginButton() {
  const { login } = useAuth();
  const [loginStatus, setLoginStatus] = useState<LoginStatus>("idle");

  const handleLogin = async () => {
    setLoginStatus("logging-in");
    try {
      await login();
      setLoginStatus("success");
    } catch (e) {
      console.error(e);
      setLoginStatus("error");
    }
  };

  const className = "w-full";
  const button: Record<LoginStatus, ReactElement> = {
    error: (
      <Button disabled size={"lg"} className={className}>
        <CircleX className="inline-block w-5 m-0 mr-2" />
        Error
      </Button>
    ),
    "logging-in": (
      <Button disabled size={"lg"} className={className}>
        <LoaderCircle className="inline-block w-5 m-0 mr-2 animate-spin" /> Signing
        in ...
      </Button>
    ),
    success: (
      <Button disabled size={"lg"} className={className}>
        <CheckCircle className="inline-block w-5 m-0 mr-2" /> Signed in
      </Button>
    ),
    idle: (
      <Button onClick={handleLogin} size={"lg"} className={className}>
        <img src={DfinityLogo} className="inline-block w-5 m-0 mr-2" /> Sign in
      </Button>
    ),
  };

  return button[loginStatus];
}
