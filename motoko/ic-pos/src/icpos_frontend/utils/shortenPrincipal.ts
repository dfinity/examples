import { Principal } from "@dfinity/principal";

export const shortenPrincipal = (principal: string | Principal | undefined) => {
  if (!principal) return "";
  if (typeof principal !== "string") principal = principal.toString();
  const parts = principal.split("-");
  return parts[0] + "..." + parts[parts.length - 1];
};
