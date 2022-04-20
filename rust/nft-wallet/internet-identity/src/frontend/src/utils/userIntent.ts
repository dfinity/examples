// What does the user intend to do after logging in?
export type UserIntent = ManageIntent | AuthIntent | AddDeviceIntent;

export type ManageIntent = { kind: "manage" };
export type AuthIntent = { kind: "auth" };
export type AddDeviceIntent = { kind: "addDevice" };

export const intentFromUrl = (url: URL): UserIntent => {
  if (url.hash == "#authorize") {
    return { kind: "auth" };
  } else if (url.hash?.split("device=")[1] !== undefined) {
    return { kind: "addDevice" };
  } else {
    return { kind: "manage" };
  }
};

export const authenticateIntent = (intent: UserIntent): string => {
  switch (intent.kind) {
    case "addDevice":
      return "Authenticate to add your new device";
    case "auth":
      return "Authenticate using Internet Identity";
    case "manage":
      return "Authenticate using Internet Identity";
  }
};

// TODO: Remove me
export const authenticateUnknownIntent = (intent: UserIntent): string => {
  switch (intent.kind) {
    case "addDevice":
      return " and to add your new device";
    case "auth":
      return " using your Internet Identity";
    case "manage":
      return "";
  }
};
