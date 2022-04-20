class View {
  constructor(protected browser: WebdriverIO.Browser) {}
}

export class WelcomeView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#registerUserNumber")
      .waitForDisplayed({ timeout: 10_000 });
  }

  async typeUserNumber(userNumber: string): Promise<void> {
    await this.browser.$("#registerUserNumber").setValue(userNumber);
  }

  async login(): Promise<void> {
    await this.browser.$("#loginButton").click();
  }

  async register(): Promise<void> {
    await this.browser.$("#registerButton").click();
  }

  async addDevice(): Promise<void> {
    await this.browser.$("#addNewDeviceButton").click();
  }

  async recover(): Promise<void> {
    await this.browser.$("#recoverButton").click();
  }
}

export class RegisterView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#registerAlias")
      .waitForDisplayed({ timeout: 10_000 });
  }

  async enterAlias(alias: string): Promise<void> {
    await this.browser.$("#registerAlias").setValue(alias);
  }

  async create(): Promise<void> {
    await this.browser.$('button[type="submit"]').click();
  }

  // View: Register confirmation
  async waitForRegisterConfirm(): Promise<void> {
    await this.browser
      .$("#confirmRegisterButton")
      .waitForDisplayed({ timeout: 25_000 });
    await this.browser.$("#captchaInput").waitForDisplayed({ timeout: 25_000 });
  }

  async confirmRegisterConfirm(): Promise<void> {
    await this.browser.$("#captchaInput").waitForEnabled({ timeout: 40_000 });
    // In tests, the captchas are hard-coded to the following string: "a"
    await this.browser.$("#captchaInput").setValue("a");
    await this.browser
      .$("#confirmRegisterButton")
      // this is a huge timeout because generating the captcha takes a while on
      // the emulator
      .waitForEnabled({ timeout: 30_000 });
    await this.browser.$("#confirmRegisterButton").click();
  }

  // View: Register Show Number
  async waitForIdentity(): Promise<void> {
    await this.browser
      .$("#displayUserContinue")
      .waitForDisplayed({ timeout: 15_000 });
  }

  async registerGetIdentity(): Promise<string> {
    return await this.browser.$(".highlightBox").getText();
  }

  async registerConfirmIdentity(): Promise<void> {
    await this.browser.$("#displayUserContinue").click();
  }

  async registerIdentityFixup(): Promise<void> {
    const elem = await this.browser.$(".highlightBox");
    await this.browser.execute(
      "arguments[0].innerText = arguments[1];",
      elem,
      "12345"
    );
  }
}

export class SingleDeviceWarningView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#displayWarningAddRecovery")
      .waitForDisplayed({ timeout: 10_000 });
    await this.browser
      .$("#displayWarningRemindLater")
      .waitForDisplayed({ timeout: 1_000 });
  }

  async addRecovery(): Promise<void> {
    // we need to scroll down in case of NOT headless, otherwise the button may not be visible
    await this.browser.execute(
      "window.scrollTo(0, document.body.scrollHeight)"
    );
    await this.browser.$("#displayWarningAddRecovery").click();
  }

  async remindLater(): Promise<void> {
    // we need to scroll down in case of NOT headless, otherwise the button may not be visible
    await this.browser.execute(
      "window.scrollTo(0, document.body.scrollHeight)"
    );
    await this.browser.$("#displayWarningRemindLater").click();
  }
}

export class RecoveryMethodSelectorView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser.$("#skipRecovery").waitForDisplayed({ timeout: 10_000 });
  }

  async useSeedPhrase(): Promise<void> {
    await this.browser.$("#seedPhrase").click();
  }

  async waitForSeedPhrase(): Promise<void> {
    await this.browser
      .$("//h1[string()='Seedphrase']")
      .waitForDisplayed({ timeout: 15_000 });
  }

  async getSeedPhrase(): Promise<string> {
    return await this.browser.$("#seedPhrase").getText();
  }

  async skipRecovery(): Promise<void> {
    await this.browser.$("#skipRecovery").click();
  }

  async copySeedPhrase(): Promise<void> {
    await this.browser.$("#seedCopy").click();
  }

  async seedPhraseContinue(): Promise<void> {
    await this.browser.$("#displaySeedPhraseContinue").click();
  }
}

export class MainView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("//h1[string()='Anchor Management']")
      .waitForDisplayed({ timeout: 5_000 });
  }

  async waitForDeviceDisplay(deviceName: string): Promise<void> {
    await this.browser
      .$(`//div[string()='${deviceName}']`)
      .waitForDisplayed({ timeout: 10_000 });
  }

  async addAdditionalDevice(): Promise<void> {
    await this.browser.$("#addAdditionalDevice").click();
  }

  async logout(): Promise<void> {
    await this.browser.$("#logoutButton").click();
  }

  async addRecovery(): Promise<void> {
    await this.browser.$("#addRecovery").click();
  }

  async fixup(): Promise<void> {
    const elem = await this.browser.$(".highlightBox");
    await this.browser.execute(
      "arguments[0].innerText = arguments[1];",
      elem,
      "12345"
    );
  }

  async removeDevice(deviceName: string): Promise<void> {
    await this.browser
      .$(`//div[string()='${deviceName}']/following-sibling::button`)
      .click();
  }
}

export class AddDeviceAliasView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#deviceAliasContinue")
      .waitForDisplayed({ timeout: 3_000 });
  }

  async addAdditionalDevice(alias: string): Promise<void> {
    await this.browser.$("#deviceAlias").setValue(alias);
  }

  async continue(): Promise<void> {
    await this.browser.$("#deviceAliasContinue").click();
  }
}

export class AuthorizeAppView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#confirmRedirect")
      .waitForDisplayed({ timeout: 5_000 });
  }

  async confirm(): Promise<void> {
    await this.browser.$("#confirmRedirect").click();
  }
}

export class WelcomeBackView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#loginDifferent")
      .waitForDisplayed({ timeout: 15_000 });
  }

  async getIdentityAnchor(): Promise<string> {
    return await this.browser.$(".highlightBox").getText();
  }

  async login(): Promise<void> {
    await this.browser.$("#login").click();
  }

  async fixup(): Promise<void> {
    const elem = await this.browser.$(".highlightBox");
    await this.browser.execute(
      "arguments[0].innerText = arguments[1];",
      elem,
      "12345"
    );
  }
}

export class AddIdentityAnchorView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#addDeviceUserNumber")
      .waitForDisplayed({ timeout: 3_000 });
  }

  async continue(userNumber?: string): Promise<void> {
    if (userNumber !== undefined) {
      await fillText(this.browser, "addDeviceUserNumber", userNumber);
    }
    await this.browser.$("#addDeviceUserNumberContinue").click();
  }

  async fixup(): Promise<void> {
    // replace the Identity Anchor for a reproducible screenshot
    const elem = await this.browser.$("#addDeviceUserNumber");
    await this.browser.execute(
      "arguments[0].value = arguments[1];",
      elem,
      "12345"
    );
  }
}

export class AddDeviceView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser.$("#linkText").waitForDisplayed({ timeout: 3_000 });
  }

  async getLinkText(): Promise<string> {
    return await this.browser.$("#linkText").getAttribute("value");
  }

  async fixup(): Promise<void> {
    await this.browser.$("#linkText").waitForDisplayed({ timeout: 3_000 });
    const elem = await this.browser.$("#linkText");
    await this.browser.execute(
      "arguments[0].value = arguments[1];",
      elem,
      "(link removed from screenshot)"
    );
  }

  // View: Add device confirm
  async waitForConfirmDisplay(): Promise<void> {
    await this.browser.$("#addDevice").waitForDisplayed({ timeout: 3_000 });
  }

  async confirm(): Promise<void> {
    await this.browser.$("#addDevice").click();
  }

  async fixupConfirm(): Promise<void> {
    const userNumberElem = await this.browser.$(".highlightBox");
    await this.browser.execute(
      "arguments[0].innerText = arguments[1];",
      userNumberElem,
      "12345"
    );
  }

  // View: Add device alias
  async waitForAliasDisplay(): Promise<void> {
    await this.browser
      .$("#deviceAliasContinue")
      .waitForDisplayed({ timeout: 3_000 });
  }

  async addDeviceAlias(alias: string): Promise<void> {
    await this.browser.$("#deviceAlias").setValue(alias);
  }

  async addDeviceAliasContinue(): Promise<void> {
    await this.browser.$("#deviceAliasContinue").click();
  }

  // View: Add device success
  async waitForAddDeviceSuccess(): Promise<void> {
    await this.browser
      .$("#manageDevicesButton")
      .waitForDisplayed({ timeout: 10_000 });
  }
}

export class AboutView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("//h1[string()='About']")
      .waitForDisplayed({ timeout: 5_000 });
  }
}

export class CompatabilityNoticeView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("#compatibilityNotice")
      .waitForDisplayed({ timeout: 3_000 });
  }
}

export class DemoAppView extends View {
  async open(demoAppUrl: string, iiUrl: string): Promise<void> {
    await this.browser.url(demoAppUrl);
    await fillText(this.browser, "iiUrl", iiUrl);
  }

  async waitForDisplay(): Promise<void> {
    await this.browser.$("#principal").waitForDisplayed({ timeout: 10_000 });
  }

  async getPrincipal(): Promise<string> {
    return await this.browser.$("#principal").getText();
  }

  async signin(): Promise<void> {
    await this.browser.$("#signinBtn").click();
  }

  async setMaxTimeToLive(mttl: BigInt): Promise<void> {
    await fillText(this.browser, "maxTimeToLive", String(mttl));
  }

  async whoami(replicaUrl: string, whoamiCanister: string): Promise<string> {
    await fillText(this.browser, "hostUrl", replicaUrl);
    await fillText(this.browser, "canisterId", whoamiCanister);
    await this.browser.$("#whoamiBtn").click();
    const whoamiResponseElem = await this.browser.$("#whoamiResponse");
    await whoamiResponseElem.waitUntil(
      async () => {
        return (await whoamiResponseElem.getText()).indexOf("-") !== -1;
      },
      {
        timeout: 6_000,
        timeoutMsg: 'expected whoami response to contain "-" for 6s',
      }
    );
    return await whoamiResponseElem.getText();
  }
}

export class RecoverView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$(`//h1[string()='Recover Identity Anchor']`)
      .waitForDisplayed({ timeout: 5_000 });
  }

  async enterIdentityAnchor(identityAnchor: string): Promise<void> {
    await this.browser.$("#userNumberInput").setValue(identityAnchor);
  }

  async continue(): Promise<void> {
    await this.browser.$("#userNumberContinue").click();
  }

  // enter seed phrase view
  async waitForSeedInputDisplay(): Promise<void> {
    await this.browser
      .$(`//h1[string()='Your seed phrase']`)
      .waitForDisplayed({ timeout: 5_000 });
  }

  async enterSeedPhrase(seedPhrase: string): Promise<void> {
    await this.browser.$("#inputSeedPhrase").setValue(seedPhrase);
  }

  async enterSeedPhraseContinue(): Promise<void> {
    await this.browser.$("#inputSeedPhraseContinue").click();
  }
}

export class FAQView extends View {
  async waitForDisplay(): Promise<void> {
    await this.browser
      .$("//h1[string()='FAQ']")
      .waitForDisplayed({ timeout: 5_000 });
  }

  async openQuestion(questionAnchor: string): Promise<void> {
    await this.browser.$(`#${questionAnchor} summary`).click();
  }
}

async function fillText(
  browser: WebdriverIO.Browser,
  id: string,
  text: string
): Promise<void> {
  const elem = await browser.$(`#${id}`);
  await elem.clearValue();
  await elem.setValue(text);
}
