import { Link, Navigate } from "@tanstack/router";

import { Button } from "../../components/ui/button";
import ConfigForm from "./components/ConfigForm";
import HeaderSection from "../../components/HeaderSection";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";
import { X } from "lucide-react";
import { useAuth } from "../../auth/hooks/useAuth";

export default function ConfigPage() {
  const { hasLoggedIn } = useAuth();

  // This page requires authentication
  if (!hasLoggedIn) {
    return <Navigate to="/" />;
  }

  return (
    <Page>
      <HeaderSection>
        <Link to="/merchant">
          <Button variant="ghost" size="icon">
            <X className="w-4 h-4" />
          </Button>
        </Link>
        <span>Configure Store</span>
        <div className="w-4 h-4" />
      </HeaderSection>
      <MainSection>
        <div className="flex flex-col items-center justify-between p-5 pb-10 space-y-5 grow">
          <div className="grow" />
          <ConfigForm />
        </div>
      </MainSection>
    </Page>
  );
}
