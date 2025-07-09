import DfinityLogo from "../../assets/dfinity-logo.png";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Button } from "../ui/button";
import { CheckCircle, CircleX, LoaderCircle } from "lucide-react";
import { useEffect } from "react";
import { router } from "@/main";

export default function LoginButton() {
  const { login, loginStatus, isLoginSuccess } = useInternetIdentity();

  useEffect(() => {
    if (!isLoginSuccess) return;
    router.invalidate();
  }, [isLoginSuccess]);

  const className = "w-full"
  const button = {
    'error': <Button disabled size={"lg"} className={className}>
      <CircleX className="inline-block w-5 m-0 mr-2" />Error
    </Button>
    ,
    'logging-in': <Button disabled size={"lg"} className={className}>
      <LoaderCircle className="inline-block w-5 m-0 mr-2 animate-spin" /> Signing in ...
    </Button>
    ,
    'success': <Button disabled size={"lg"} className={className}>
      <CheckCircle className="inline-block w-5 m-0 mr-2" /> Signed in
    </Button>
    ,
    'idle': <Button onClick={login} size={"lg"} className={className}>
      <img src={DfinityLogo} className="inline-block w-5 m-0 mr-2" /> Sign in
    </Button>

  }

  return button[loginStatus]
}
