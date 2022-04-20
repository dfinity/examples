import { IIConnection, ApiResult } from "../../utils/iiConnection";

export type LoginFlowResult =
  | {
      tag: "ok";
      userNumber: bigint;
      connection: IIConnection;
    }
  | {
      tag: "err";
      title: string;
      message: string;
      detail?: string;
    };

export const apiResultToLoginFlowResult = (
  result: ApiResult
): LoginFlowResult => {
  switch (result.kind) {
    case "loginSuccess": {
      return {
        tag: "ok",
        userNumber: result.userNumber,
        connection: result.connection,
      };
    }
    case "authFail": {
      return {
        tag: "err",
        title: "Failed to authenticate",
        message:
          "We failed to authenticate you using your security device. If this is the first time you're trying to log in with this device, you have to add it as a new device first.",
        detail: result.error.message,
      };
    }
    case "unknownUser": {
      return {
        tag: "err",
        title: "Unknown Identity Anchor",
        message: `Failed to find an identity for the Identity Anchor ${result.userNumber}. Please check your Identity Anchor and try again.`,
        detail: "",
      };
    }
    case "apiError": {
      return {
        tag: "err",
        title: "We couldn't reach Internet Identity",
        message:
          "We failed to call the Internet Identity service, please try again.",
        detail: result.error.message,
      };
    }
    case "registerNoSpace": {
      return {
        tag: "err",
        title: "Failed to register",
        message:
          "Failed to register with Internet Identity, because there is no space left at the moment. We're working on increasing the capacity.",
      };
    }
    case "badChallenge": {
      return {
        tag: "err",
        title: "Failed to register",
        message:
          "Failed to register with Internet Identity, because the CAPTCHA challenge wasn't successful",
      };
    }
    case "seedPhraseFail": {
      return {
        tag: "err",
        title: "Invalid Seed Phrase",
        message:
          "Failed to recover using this seedphrase. Did you enter it correctly?",
      };
    }
  }
};
