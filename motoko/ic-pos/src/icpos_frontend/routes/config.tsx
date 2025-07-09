import { createFileRoute, Link, redirect } from '@tanstack/react-router'
import Page from '@/components/Page';
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import MainSection from '@/components/MainSection';
import ConfigForm from '@/components/config/ConfigForm';
import { X } from 'lucide-react';

export const Route = createFileRoute('/config')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },
  component: ConfigPage,
})

function ConfigPage() {
  return (
    <Page>
      <HeaderSection>
        <Link to="/">
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
