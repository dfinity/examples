import DfinityLogo from "../assets/dfinity-logo.png";
import { Principal } from "@icp-sdk/core/principal";
import { principalToString } from "../utils/principalToString";
import { shortenPrincipal } from "../utils/shortenPrincipal";
import { copyToClipboard } from "../utils/clipboard";
import { toast } from "react-hot-toast";

async function copyPrincipal(principal: string | Principal | undefined) {
  if (!principal) return;
  const copied = await copyToClipboard(principalToString(principal));
  if (copied) {
    toast.success("Copied");
  } else {
    toast.error("Couldn't copy to clipboard");
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
      className={`not-prose inline-flex items-center gap-1.5 py-1 px-3 bg-black/10 rounded-full text-[0.9rem] hover:bg-black/20 cursor-pointer ${className ?? ""}`}
      onClick={() => void copyPrincipal(principal)}
    >
      <img src={DfinityLogo} alt="" className="h-4 w-4 shrink-0 object-contain m-0" />
      <span>
        {variant === "short"
          ? shortenPrincipal(principal)
          : principalToString(principal)}
      </span>
    </div>
  );
}
