import App from "./App.svelte";
import { default as vetkd_init } from "../../../vetkd_user_lib/ic_vetkd_utils.js";
import vetkd_wasm from "../../../vetkd_user_lib/ic_vetkd_utils_bg.wasm";

const init = async () => {
  // Once the wasm is initialized in this way, i.e., with the defaultExport of the respective .js file,
  // the (non-defaultExport-ed) methods of the .js file can be imported and used.
  // See also https://github.com/rollup/plugins/tree/master/packages/wasm#using-with-wasm-bindgen-and-wasm-pack
  await vetkd_init(await vetkd_wasm());

  const app = new App({
    target: document.body,
  });
};

init();