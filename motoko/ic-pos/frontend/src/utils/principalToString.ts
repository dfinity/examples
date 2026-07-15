import { Principal } from "@icp-sdk/core/principal";

export const principalToString = (
  principal: string | Principal | undefined
) => {
  if (!principal) return "";
  if (typeof principal !== "string") principal = principal.toString();
  return principal;
};
