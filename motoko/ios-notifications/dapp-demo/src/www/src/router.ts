import { Page, PageOptions } from "./types";
import { LoginPage } from "./pages/login";
import { HomePage } from "./pages/home";
import { RouteName } from "./pages";
import { Auth, MultiPlatformLoggedInAction } from "./auth";
import { AboutPage } from "./pages/about";

export class Router {
  routeParam = "route";
  routes: Record<RouteName, Page> = {
    [RouteName.home]: new HomePage(),
    [RouteName.login]: new LoginPage(),
    [RouteName.about]: new AboutPage(),
  };

  renderPage(route: RouteName, options: PageOptions): void {
    this.routes[route].render(options);
  }

  async start(auth: Auth): Promise<void> {
    const isAuthenticated = await auth.client().isAuthenticated();

    if (isAuthenticated) {
        const action = await auth.handleMultiPlatformLogin();
        if (action === MultiPlatformLoggedInAction.Redirecting) {
          return;
        }

        const url = new URL(window.location.href);
        let routeName = url.searchParams.get(this.routeParam) ?? "";
        if (routeName === RouteName.login) {
            routeName = RouteName.home;
        }

        const route = this.routes[routeName] ? routeName as RouteName : RouteName.home;

        this.renderPage(route, { auth, router: this });
        return;
    }

    this.renderPage(RouteName.login, { auth, router: this });
  }
}
