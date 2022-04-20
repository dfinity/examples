import { DeviceData } from "../../../generated/internet_identity_types";
import { displayError } from "../../components/displayError";
import { IIConnection, LoginResult } from "../../utils/iiConnection";
import { hasOwnProperty } from "../../utils/utils";
import { apiResultToLoginFlowResult } from "../login/flowResult";
import { renderManage } from "../manage";
import { promptUserNumber } from "../promptUserNumber";
import { inputSeedPhrase } from "./inputSeedPhrase";
import { pickRecoveryDevice } from "./pickRecoveryDevice";

const wantsSeedPhrase = (device: DeviceData): boolean => {
  return hasOwnProperty(device.key_type, "seed_phrase");
};

export const useRecovery = async (userNumber?: bigint): Promise<void> => {
  userNumber =
    userNumber === undefined
      ? await promptUserNumber("Recover Identity Anchor", null)
      : userNumber;
  const recoveryDevices = await IIConnection.lookupRecovery(userNumber);
  if (recoveryDevices.length === 0) {
    await displayError({
      title: "Failed to recover",
      message:
        "You do not have any recovery devices configured. Did you mean to authenticate with one of your devices instead?",
      primaryButton: "Go back",
    });
    return window.location.reload();
  }

  const recoveryDevice =
    recoveryDevices.length === 1
      ? recoveryDevices[0]
      : await pickRecoveryDevice(recoveryDevices);

  const logiFlowResult = apiResultToLoginFlowResult(
    await loginWithRecovery(userNumber, recoveryDevice)
  );
  switch (logiFlowResult.tag) {
    case "ok": {
      return renderManage(logiFlowResult.userNumber, logiFlowResult.connection);
    }
    case "err": {
      // TODO Display a recovery specific error
      await displayError({ ...logiFlowResult, primaryButton: "Try again" });
      return useRecovery();
    }
  }
};

const loginWithRecovery = async (
  userNumber: bigint,
  device: DeviceData
): Promise<LoginResult> => {
  if (wantsSeedPhrase(device)) {
    const seedPhrase = await inputSeedPhrase(userNumber);
    if (seedPhrase === null) {
      return { kind: "seedPhraseFail" };
    }
    return await IIConnection.fromSeedPhrase(userNumber, seedPhrase, device);
  } else {
    return IIConnection.fromWebauthnDevices(userNumber, [device]);
  }
};
