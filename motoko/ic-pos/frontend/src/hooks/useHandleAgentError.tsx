import { useAuth } from "@/lib/auth";
import toast from "react-hot-toast";

export default function useHandleAgentError() {
  const { clear } = useAuth();

  const handleAgentError = (e: unknown) => {
    if (
      e &&
      typeof e === "object" &&
      "message" in e &&
      typeof e.message === "string" &&
      e.message.includes("delegation has expired")
    ) {
      void clear();
      toast.error("Login expired, please login again.");
    }
  };

  return { handleAgentError };
}
