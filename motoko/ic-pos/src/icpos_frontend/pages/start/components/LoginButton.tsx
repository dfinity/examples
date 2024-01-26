import { Button } from "../../../components/ui/button";
import { useAuth } from "../../../auth/hooks/useAuth";
import DfinityLogo from "../../../assets/dfinity-logo.png";

export default function LoginButton() {
  const { login } = useAuth();

  return (
    <Button onClick={login} size={"lg"} className="w-full">
      <img src={DfinityLogo} className="inline-block w-5 m-0 mr-2" /> Sign in
    </Button>
  );
}
