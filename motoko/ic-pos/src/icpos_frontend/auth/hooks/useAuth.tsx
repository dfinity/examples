import { AuthContext } from "../context/AuthProvider";
import { AuthContextType } from "../types/auth-context.type";
import { useContext } from "react";

export const useAuth = (): Partial<AuthContextType> => useContext(AuthContext);
