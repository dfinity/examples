import HeaderSection from "../../components/HeaderSection";
import { HeartCrack } from "lucide-react";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";

export default function NotFoundPage() {
  return (
    <Page>
      <HeaderSection>
        <div />
        Not found
        <div />
      </HeaderSection>
      <MainSection>
        <div className="flex flex-col items-center justify-center p-5 pb-10 space-y-5 grow">
          <HeartCrack className="w-32 h-32" />
        </div>
      </MainSection>
    </Page>
  );
}
