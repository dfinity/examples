import App from "./App.svelte";
import page from "page";
import { fetchNftRouter } from "./nft.js";

const app = new App({
  target: document.body,
  props: {
    page: {},
  },
});

page("/", () => {
  app.$set({ page: {} });
});
page("/register", () => {
  app.$set({ page: { register: true } });
});
page("/transactions", () => {
  app.$set({ page: { transactions: true } });
});
page("/:canister/:index", fetchNftRouter("canister", "index"), (context) => {
  const params = { nft: context.state.nft };
  const fragmentIndex = context.canonicalPath.indexOf("#");
  if (fragmentIndex !== -1) {
    const fragment = context.canonicalPath.substring(fragmentIndex + 1);
    const selected = Number.parseInt(fragment);
    if (!Number.isNaN(selected)) {
      params.nftCurrent = selected;
    }
  }
  app.$set({ page: params });
});
page("/:canister", (context) => {
  app.$set({ page: { collection: context.params.canister } });
});
page(
  "/:canister/:index/transfer",
  fetchNftRouter("canister", "index"),
  (context) => {
    if (context.state.nftError) {
      app.$set({ page: { nftError: context.state.nftError } });
    } else {
      app.$set({ page: { transfer: context.state.nft } });
    }
  }
);
page();

export default app;
