import { createFileRoute } from '@tanstack/react-router'
import Page from '@/components/Page';
import HeaderSection from '@/components/HeaderSection';
import MainSection from '@/components/MainSection';
import ConfigForm from '@/components/setup/ConfigForm';
import LogoutButton from '@/components/LogoutButton';

export const Route = createFileRoute('/setup')({
  component: SetupPage,
})

function SetupPage() {
  return (
    <Page>
      <HeaderSection>
        <LogoutButton />
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

