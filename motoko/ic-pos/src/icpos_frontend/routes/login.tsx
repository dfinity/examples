import { createFileRoute, redirect } from '@tanstack/react-router'
import Page from '../components/Page'
import MainSection from '../components/MainSection'
import LoginButton from '../components/start/LoginButton'
import ckBtc from '../assets/ckBTC.svg'

export const Route = createFileRoute('/login')({
  beforeLoad: ({ context }) => {
    if (context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },

  component: LoginPage
})

function LoginPage() {
  return (
    <Page>
      <MainSection>
        <div className="flex flex-col items-center justify-between p-10 space-y-5 grow">
          <div className="flex items-center justify-center w-full p-10">
            <img src={ckBtc} />
          </div>
          <div className="text-4xl font-bold">IC-POS </div>
          <div className="text-center">
            Setup a simple store front to accept ckBTC payments on the Internet
            Computer. Sign in with Internet Identity to get started.
          </div>
          <div className="grow" />
          <LoginButton />
        </div>
      </MainSection>
    </Page>
  )
}
