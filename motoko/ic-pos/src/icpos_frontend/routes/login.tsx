import { Link, createFileRoute, redirect } from '@tanstack/react-router'
import Page from '../components/Page'
import MainSection from '../components/MainSection'
import LoginButton from '../components/start/LoginButton'

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

import ckBtc from '../assets/ckBTC.svg'

function LoginPage() {
  // const { merchantState } = useIcPos();
  //
  // // If the merchant state is initialized it means that the backend icpos actor is available
  // if (merchantState.initialized) {
  //   // If the merchant is initialized, navigate to the merchant page
  //   if (merchantState.merchant) {
  //     return <Navigate to="/merchant" />;
  //   }
  //   // Otherwise, navigate to the config page to create a merchant
  //   return <Navigate to="/initial-config" />;
  // }

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
            Computer. Sign in with Internet Identity to get started or{' '}
            <Link to="/receive-select-principal">
              monitor any address without signing in
            </Link>
            .
          </div>
          <div className="grow" />
          <LoginButton />
        </div>
      </MainSection>
    </Page>
  )
}
