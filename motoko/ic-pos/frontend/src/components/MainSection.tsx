import { ReactNode } from "react";

export default function MainSection({ children }: { children: ReactNode }) {
  return <div className="flex flex-col grow">{children}</div>;
}
