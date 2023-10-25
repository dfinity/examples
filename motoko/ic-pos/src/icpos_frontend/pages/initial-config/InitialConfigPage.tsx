import { Button } from "../../components/ui/button";
import ConfigForm from "./components/ConfigForm";
import HeaderSection from "../../components/HeaderSection";
import { LogOut } from "lucide-react";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";

export default function InitialConfigPage() {
  return (
    <Page>
      <HeaderSection>
        <Button variant="ghost" size="icon">
          <LogOut
            className="w-4 h-4"
            onClick={() => {
              window.location.href = "/";
            }}
          />
        </Button>
        <span>Configure Store</span>
        <div className="w-4 h-4" />
      </HeaderSection>
      <MainSection>
        <div className="flex flex-col items-center justify-between p-5 pb-10 space-y-5 grow">
          <div>
            Before you begin accepting payments, give your store a name!
          </div>
          <div className="grow" />
          <ConfigForm />
        </div>
      </MainSection>
    </Page>
  );
}
