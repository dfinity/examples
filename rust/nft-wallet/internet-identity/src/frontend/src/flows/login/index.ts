import { displayError } from "../../components/displayError";
import { IIConnection } from "../../utils/iiConnection";
import { UserIntent } from "../../utils/userIntent";
import { getUserNumber } from "../../utils/userNumber";
import { unknownToString } from "../../utils/utils";
import { loginUnknownAnchor } from "../login/unknownAnchor";
import { loginKnownAnchor } from "../login/knownAnchor";
import { LoginFlowResult } from ".././login/flowResult";

// We retry logging in until we get a successful Identity Anchor connection pair
// If we encounter an unexpected error we reload to be safe
export const login = async (
  userIntent: UserIntent
): Promise<{
  userNumber: bigint;
  connection: IIConnection;
}> => {
  try {
    const x = await tryLogin(userIntent);

    switch (x.tag) {
      case "ok": {
        return { userNumber: x.userNumber, connection: x.connection };
      }
      case "err": {
        await displayError({ ...x, primaryButton: "Try again" });
        return login(userIntent);
      }
    }
  } catch (err: unknown) {
    await displayError({
      title: "Something went wrong",
      message:
        "An unexpected error occurred during authentication. Please try again",
      detail: unknownToString(err, "Unknown error"),
      primaryButton: "Try again",
    });
    window.location.reload();
    return Promise.reject(err);
  }
};

const tryLogin = async (userIntent: UserIntent): Promise<LoginFlowResult> => {
  const userNumber = getUserNumber();
  if (userNumber === undefined) {
    return loginUnknownAnchor(userIntent);
  } else {
    return loginKnownAnchor(userIntent, userNumber);
  }
};
