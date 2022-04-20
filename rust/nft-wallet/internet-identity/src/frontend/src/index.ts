import "./styles/main.css";
import { login } from "./flows/login";
import auth from "./auth";
import { addDevice } from "./flows/addDevice";
import { renderManage } from "./flows/manage";
import { compatibilityNotice } from "./flows/compatibilityNotice";
import { aboutView } from "./flows/about";
import { faqView } from "./flows/faq";
import { intentFromUrl } from "./utils/userIntent";
import { hasRequiredFeatures } from "./utils/featureDetection";
import { recoveryWizard } from "./flows/recovery/recoveryWizard";

const init = async () => {
  const url = new URL(document.URL);

  // Custom routing to the FAQ page
  if (window.location.pathname === "/faq") {
    return faqView();
  }

  if (window.location.pathname === "/about" || url.hash === "#about") {
    return aboutView();
  }

  if (!(await hasRequiredFeatures(url))) {
    return compatibilityNotice();
  }

  const userIntent = intentFromUrl(url);

  // Go through the login flow, potentially creating an anchor.
  const { userNumber, connection } = await login(userIntent);

  // From here on, the user is authenticated to II.

  // Here, if the user doesn't have any recovery device, we prompt them to add
  // one. The exact flow depends on the device they use.
  await recoveryWizard(userNumber, connection);

  switch (userIntent.kind) {
    // Authenticate to a third party service
    case "auth": {
      return auth(userNumber, connection);
    }

    // Add a device
    case "addDevice": {
      return addDevice(userNumber, connection);
    }

    // Open the management page
    case "manage": {
      return renderManage(userNumber, connection);
    }
  }
};

init();
