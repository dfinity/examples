import {
  AboutView,
  AddDeviceAliasView,
  AddDeviceView,
  AddIdentityAnchorView,
  AuthorizeAppView,
  CompatabilityNoticeView,
  DemoAppView,
  FAQView,
  MainView,
  RecoverView,
  RecoveryMethodSelectorView,
  RegisterView,
  SingleDeviceWarningView,
  WelcomeBackView,
  WelcomeView,
} from "./views";
import { FLOWS } from "./flows";
import {
  Screenshots,
  addVirtualAuthenticator,
  removeVirtualAuthenticator,
  runInBrowser,
  runInNestedBrowser,
  switchToPopup,
  waitForFonts,
  waitToClose,
  setupSeleniumServer,
  RunConfiguration,
} from "./util";

// Read canister ids from the corresponding dfx files.
// This assumes that they have been successfully dfx-deployed
import canister_ids1 from "../../../../.dfx/local/canister_ids.json";
import canister_ids2 from "../../../../demos/whoami/.dfx/local/canister_ids.json";

const IDENTITY_CANISTER = canister_ids1.internet_identity.local;
const WHOAMI_CANISTER = canister_ids2.whoami.local;

const REPLICA_URL = "http://localhost:8000";
const II_URL = `http://localhost:8000/?canisterId=${IDENTITY_CANISTER}`;
const FAQ_URL = `http://localhost:8000/faq?canisterId=${IDENTITY_CANISTER}`;
const ABOUT_URL = `http://localhost:8000/about?canisterId=${IDENTITY_CANISTER}`;
const DEMO_APP_URL = "http://localhost:8080/";

const DEVICE_NAME1 = "Virtual WebAuthn device";
const DEVICE_NAME2 = "Other WebAuthn device";

setupSeleniumServer();

test("Register new identity and login with it", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await browser.url(II_URL);
    const welcomeView = new WelcomeView(browser);
    await welcomeView.waitForDisplay();
    await welcomeView.register();
    await addVirtualAuthenticator(browser);
    await browser.url(II_URL);
    const userNumber = await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const mainView = new MainView(browser);
    await mainView.waitForDeviceDisplay(DEVICE_NAME1);
    await mainView.logout();
    await FLOWS.login(userNumber, DEVICE_NAME1, browser);
  });
}, 300_000);

test("Register new identity and add additional device", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    const firstAuthenticator = await addVirtualAuthenticator(browser);
    await browser.url(II_URL);
    const userNumber = await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);

    const mainView = new MainView(browser);
    await mainView.waitForDeviceDisplay(DEVICE_NAME1);
    // We're removing the first authenticator here, because unfortunately we
    // can't tell Chrome to _actually_ use the second authenticator, which
    // leads to flaky tests otherwise.
    await removeVirtualAuthenticator(browser, firstAuthenticator);
    await addVirtualAuthenticator(browser);
    await mainView.addAdditionalDevice();

    const addDeviceAliasView = new AddDeviceAliasView(browser);
    await addDeviceAliasView.waitForDisplay();
    await addDeviceAliasView.addAdditionalDevice(DEVICE_NAME2);
    await addDeviceAliasView.continue();

    await browser.pause(10_000);

    await mainView.waitForDeviceDisplay(DEVICE_NAME1);
    await mainView.waitForDeviceDisplay(DEVICE_NAME2);

    await mainView.logout();
    await FLOWS.login(userNumber, DEVICE_NAME1, browser);
  });
}, 300_000);

test("Log into client application, after registration", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await addVirtualAuthenticator(browser);
    const demoAppView = new DemoAppView(browser);
    await demoAppView.open(DEMO_APP_URL, II_URL);
    await demoAppView.waitForDisplay();
    expect(await demoAppView.getPrincipal()).toBe("2vxsx-fae");
    await demoAppView.signin();
    await switchToPopup(browser);
    await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const authorizeAppView = new AuthorizeAppView(browser);
    await authorizeAppView.waitForDisplay();
    await authorizeAppView.confirm();
    await waitToClose(browser);
    await demoAppView.waitForDisplay();
    const principal = await demoAppView.getPrincipal();
    expect(principal).not.toBe("2vxsx-fae");

    expect(await demoAppView.whoami(REPLICA_URL, WHOAMI_CANISTER)).toBe(
      principal
    );

    // default value
    const exp = await browser.$("#expiration").getText();
    expect(Number(exp) / (24 * 60 * 60_000_000_000)).toBeCloseTo(1);
  });
}, 300_000);

test("Delegation maxTimeToLive: 1 min", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await addVirtualAuthenticator(browser);
    const demoAppView = new DemoAppView(browser);
    await demoAppView.open(DEMO_APP_URL, II_URL);
    await demoAppView.waitForDisplay();
    expect(await demoAppView.getPrincipal()).toBe("2vxsx-fae");
    await demoAppView.setMaxTimeToLive(BigInt(60_000_000_000));
    await demoAppView.signin();
    await switchToPopup(browser);
    await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const authorizeAppView = new AuthorizeAppView(browser);
    await authorizeAppView.waitForDisplay();
    await authorizeAppView.confirm();
    await waitToClose(browser);
    await demoAppView.waitForDisplay();
    expect(await demoAppView.getPrincipal()).not.toBe("2vxsx-fae");
    const exp = await browser.$("#expiration").getText();
    // compare only up to one decimal place for the 1min test
    expect(Number(exp) / 60_000_000_000).toBeCloseTo(1, 0);
  });
}, 300_000);

test("Delegation maxTimeToLive: 1 day", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await addVirtualAuthenticator(browser);
    const demoAppView = new DemoAppView(browser);
    await demoAppView.open(DEMO_APP_URL, II_URL);
    await demoAppView.waitForDisplay();
    expect(await demoAppView.getPrincipal()).toBe("2vxsx-fae");
    await demoAppView.setMaxTimeToLive(BigInt(86400_000_000_000));
    await demoAppView.signin();
    await switchToPopup(browser);
    await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const authorizeAppView = new AuthorizeAppView(browser);
    await authorizeAppView.waitForDisplay();
    await authorizeAppView.confirm();
    await waitToClose(browser);
    expect(await demoAppView.getPrincipal()).not.toBe("2vxsx-fae");
    const exp = await browser.$("#expiration").getText();
    expect(Number(exp) / 86400_000_000_000).toBeCloseTo(1);
  });
}, 300_000);

test("Delegation maxTimeToLive: 1 month", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await addVirtualAuthenticator(browser);
    const demoAppView = new DemoAppView(browser);
    await demoAppView.open(DEMO_APP_URL, II_URL);
    await demoAppView.waitForDisplay();
    expect(await demoAppView.getPrincipal()).toBe("2vxsx-fae");
    await demoAppView.setMaxTimeToLive(BigInt(2592000_000_000_000));
    await demoAppView.signin();
    await switchToPopup(browser);
    await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const authorizeAppView = new AuthorizeAppView(browser);
    await authorizeAppView.waitForDisplay();
    await authorizeAppView.confirm();
    await waitToClose(browser);
    expect(await demoAppView.getPrincipal()).not.toBe("2vxsx-fae");
    const exp = await browser.$("#expiration").getText();
    // NB: Max out at 8 days
    expect(Number(exp) / 691200_000_000_000).toBeCloseTo(1);
  });
}, 300_000);

test("Recover access, after registration", async () => {
  await runInBrowser(async (browser: WebdriverIO.Browser) => {
    await addVirtualAuthenticator(browser);
    await browser.url(II_URL);
    const userNumber = await FLOWS.registerNewIdentity(DEVICE_NAME1, browser);
    const mainView = new MainView(browser);
    await mainView.waitForDeviceDisplay(DEVICE_NAME1);
    const seedPhrase = await FLOWS.addRecoveryMechanismSeedPhrase(browser);
    await mainView.waitForDisplay();
    await mainView.logout();

    const welcomeView = new WelcomeView(browser);
    await welcomeView.recover();
    const recoveryView = new RecoverView(browser);
    await recoveryView.waitForDisplay();
    await recoveryView.enterIdentityAnchor(userNumber);
    await recoveryView.continue();
    await recoveryView.waitForSeedInputDisplay();
    await recoveryView.enterSeedPhrase(seedPhrase);
    await recoveryView.enterSeedPhraseContinue();
    await mainView.waitForDeviceDisplay(DEVICE_NAME1);
  });
}, 300_000);

test("Screenshots", async () => {
  await runInBrowser(
    async (browser: WebdriverIO.Browser, runConfig: RunConfiguration) => {
      const screenshots = new Screenshots(
        "screenshots/",
        runConfig.screenConfiguration.screenType
      );

      await addVirtualAuthenticator(browser);
      await browser.url(II_URL);

      await waitForFonts(browser);
      const welcomeView = new WelcomeView(browser);
      await welcomeView.waitForDisplay();
      await screenshots.take("welcome", browser);
      await welcomeView.register();
      const registerView = new RegisterView(browser);
      await registerView.waitForDisplay();
      await screenshots.take("register", browser);
      await registerView.enterAlias(DEVICE_NAME1);
      await registerView.create();
      await registerView.waitForRegisterConfirm();
      await screenshots.take("register-confirm", browser);
      await registerView.confirmRegisterConfirm();
      await registerView.waitForIdentity();
      const userNumber = await registerView.registerGetIdentity();
      await registerView.registerIdentityFixup();
      await screenshots.take("register-user-number", browser);
      await registerView.registerConfirmIdentity();
      const recoveryMethodSelectorView = new RecoveryMethodSelectorView(
        browser
      );
      await recoveryMethodSelectorView.waitForDisplay();
      await screenshots.take("recover-method-selector", browser);
      await recoveryMethodSelectorView.skipRecovery();
      const singleDeviceWarningView = new SingleDeviceWarningView(browser);
      await singleDeviceWarningView.waitForDisplay();
      await screenshots.take("single-device-warning", browser);
      await singleDeviceWarningView.remindLater();
      const mainView = new MainView(browser);
      await mainView.waitForDeviceDisplay(DEVICE_NAME1);
      await mainView.fixup();
      await screenshots.take("main", browser);
      await mainView.logout();
      await welcomeView.waitForDisplay(); // no point taking screenshot
      await welcomeView.typeUserNumber(userNumber);
      await welcomeView.login();
      await recoveryMethodSelectorView.waitForDisplay();
      await recoveryMethodSelectorView.skipRecovery();
      await singleDeviceWarningView.waitForDisplay();
      await singleDeviceWarningView.remindLater();
      await mainView.waitForDeviceDisplay(DEVICE_NAME1);

      await browser.url(II_URL);
      const welcomeBackView = new WelcomeBackView(browser);
      await welcomeBackView.waitForDisplay();
      const userNumber2 = await welcomeBackView.getIdentityAnchor();
      expect(userNumber2).toBe(userNumber);
      await welcomeBackView.fixup();
      await screenshots.take("welcome-back", browser);
      await welcomeBackView.login();
      await recoveryMethodSelectorView.waitForDisplay();
      await recoveryMethodSelectorView.skipRecovery();
      await singleDeviceWarningView.waitForDisplay();
      await singleDeviceWarningView.remindLater();
      await mainView.waitForDeviceDisplay(DEVICE_NAME1);

      // Now the link device flow, using a second browser
      await runInNestedBrowser(async (browser2: WebdriverIO.Browser) => {
        await addVirtualAuthenticator(browser2);
        await browser2.url(II_URL);
        const welcomeView2 = new WelcomeView(browser2);
        await welcomeView2.waitForDisplay();
        await welcomeView2.typeUserNumber(userNumber);
        await welcomeView2.addDevice();
        const addIdentityAnchorView2 = new AddIdentityAnchorView(browser2);
        await addIdentityAnchorView2.waitForDisplay();
        await addIdentityAnchorView2.fixup();
        await screenshots.take("new-device-user-number", browser2);
        await addIdentityAnchorView2.continue(userNumber);
        const addDeviceView2 = new AddDeviceView(browser2);
        await addDeviceView2.waitForDisplay();

        const link = await addDeviceView2.getLinkText();
        console.log("The add device link is", link);
        await addDeviceView2.fixup();
        await screenshots.take("new-device", browser2);

        // Log in with previous browser again
        await browser.url("about:blank");
        await browser.url(link);
        await waitForFonts(browser);
        const welcomeBackView = new WelcomeBackView(browser);
        await welcomeBackView.waitForDisplay();
        await welcomeBackView.fixup();
        await screenshots.take("new-device-login", browser);
        await welcomeBackView.login();
        await recoveryMethodSelectorView.waitForDisplay();
        await recoveryMethodSelectorView.skipRecovery();
        await singleDeviceWarningView.waitForDisplay();
        await singleDeviceWarningView.remindLater();
        const addDeviceView = new AddDeviceView(browser);
        await addDeviceView.waitForConfirmDisplay();
        await addDeviceView.fixupConfirm();
        await screenshots.take("new-device-confirm", browser);
        await addDeviceView.confirm();
        await addDeviceView.waitForAliasDisplay();
        await screenshots.take("new-device-alias", browser);
        await addDeviceView.addDeviceAlias(DEVICE_NAME2);
        await addDeviceView.addDeviceAliasContinue();
        await addDeviceView.waitForAddDeviceSuccess();
        await screenshots.take("new-device-done", browser);

        // Back to other browser, should be a welcome view now
        const welcomeBackView2 = new WelcomeBackView(browser2);
        await welcomeBackView2.waitForDisplay();
        await welcomeBackView2.fixup();
        await screenshots.take("new-device-login", browser2);
        await welcomeBackView2.login();
        const recoveryMethodSelectorView2 = new RecoveryMethodSelectorView(
          browser2
        );
        await recoveryMethodSelectorView2.waitForDisplay();
        await recoveryMethodSelectorView2.skipRecovery();
        const singleDeviceWarningView2 = new SingleDeviceWarningView(browser2);
        await singleDeviceWarningView2.waitForDisplay();
        await singleDeviceWarningView2.remindLater();
        const mainView2 = new MainView(browser2);
        await mainView2.waitForDeviceDisplay(DEVICE_NAME2);
        await mainView2.fixup();
        await screenshots.take("new-device-listed", browser2);

        // Try to remove current device
        await mainView2.removeDevice(DEVICE_NAME2);
        await browser2.waitUntil(
          async () => !!(await browser2.getAlertText()),
          {
            timeout: 1_000,
            timeoutMsg: "expected alert to be displayed after 1s",
          }
        );
        expect(await browser2.getAlertText()).toBe(
          "This will remove your current device and you will be logged out."
        );
        await browser2.dismissAlert();
        await browser2.deleteSession();
      });

      // About page
      await browser.url("about:blank");
      await browser.url(ABOUT_URL);
      await waitForFonts(browser);
      const aboutView = new AboutView(browser);
      await aboutView.waitForDisplay();
      await screenshots.take("about", browser);

      // About page, legacy link
      await browser.url("about:blank");
      await browser.url(II_URL + "#about");
      await waitForFonts(browser);
      const aboutViewLegacy = new AboutView(browser);
      await aboutViewLegacy.waitForDisplay();
      await screenshots.take("about-legacy", browser);

      // Test device removal
      await browser.url(II_URL);
      await welcomeBackView.waitForDisplay();
      const userNumber3 = await welcomeBackView.getIdentityAnchor();
      expect(userNumber3).toBe(userNumber);
      await welcomeBackView.login();
      await recoveryMethodSelectorView.waitForDisplay();
      await recoveryMethodSelectorView.skipRecovery();
      await singleDeviceWarningView.waitForDisplay();
      await singleDeviceWarningView.remindLater();
      await mainView.waitForDeviceDisplay(DEVICE_NAME2);
      await mainView.removeDevice(DEVICE_NAME2);
      // No dialog here!
      await browser.waitUntil(
        async () => {
          const device2 = await browser.$(`//div[string()='${DEVICE_NAME2}']`);
          return !(await device2.isDisplayed());
        },
        {
          timeout: 10_000,
          timeoutMsg: 'expected "Other WebAuthn device" to be gone after 10s',
        }
      );
      await mainView.waitForDeviceDisplay(DEVICE_NAME1);
      await mainView.fixup();
      await screenshots.take("after-removal", browser);

      await mainView.removeDevice(DEVICE_NAME1);
      const alertText = await browser.getAlertText();
      expect(alertText).toBe("You can not remove your last device.");
      await browser.acceptAlert();

      // device still present. You can't remove your last device.
      await mainView.waitForDeviceDisplay(DEVICE_NAME1);

      // Compatibility notice page
      await browser.url("about:blank");
      await browser.url(II_URL + "#compatibilityNotice");
      await waitForFonts(browser);
      const compatabilityNoticeView = new CompatabilityNoticeView(browser);
      await compatabilityNoticeView.waitForDisplay();
      await screenshots.take("compatibility-notice", browser);

      // FAQ page
      await browser.url("about:blank");
      await browser.url(FAQ_URL);
      await waitForFonts(browser);
      const faqView = new FAQView(browser);
      await faqView.waitForDisplay();
      await screenshots.take("faq", browser);

      // FAQ open page
      await faqView.openQuestion("lost-device");
      await faqView.waitForDisplay();
      await screenshots.take("faq-open", browser);
    }
  );
}, 400_000);
