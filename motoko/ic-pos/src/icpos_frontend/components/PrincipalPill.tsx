import DfinityLogo from "../assets/dfinity-logo.png";
import { Principal } from "@dfinity/principal";
import { principalToString } from "../utils/principalToString";
import { shortenPrincipal } from "../utils/shortenPrincipal";
import { toast } from "react-hot-toast";

function copyPrincipal(principal: string | Principal | undefined) {
  if (principal) {
    navigator.clipboard.writeText(principalToString(principal));
    toast.success("Copied");
  }
}

type PrincipalPillProps = {
  principal: string | Principal | undefined;
  className?: string;
  variant?: "short" | "full";
};

export default function PrincipalPill({
  principal,
  className,
  variant = "short",
}: PrincipalPillProps) {
  return (
    <div
      className={`py-1 px-3 bg-black rounded-full bg-opacity-10 text-[0.9rem] ${className} hover:bg-opacity-20 cursor-pointer`}
      onClick={() => copyPrincipal(principal)}
    >
      <img src={DfinityLogo} className="inline-block w-5 m-0" />{" "}
      {variant === "short"
        ? shortenPrincipal(principal)
        : principalToString(principal)}
    </div>
  );
}
