import { Auth } from "./auth";
import { Router } from "./router";

export type PageOptions = { auth: Auth, router: Router };
export interface Page {
    render(options: PageOptions): Promise<void>;
}
