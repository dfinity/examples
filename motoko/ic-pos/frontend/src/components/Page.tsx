import { ReactNode } from "react";

type PageProps = {
  children: ReactNode;
};

export default function Page(props: PageProps) {
  const { children } = props;
  return children;
}
