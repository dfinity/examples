import { Auth } from "./auth";
import { Router } from "./router";

const init = async () => {
  const router = new Router();
  const auth = await Auth.create({
    idleOptions: {
      disableIdle: true
    },
  });

  await router.start(auth);
};

init();
