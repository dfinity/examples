import { WebAuthnIdentity } from "@dfinity/identity";
import { displayError } from "../../components/displayError";
import { withLoader } from "../../components/loader";
import { fromMnemonicWithoutValidation } from "../../crypto/ed25519";
import { generate } from "../../crypto/mnemonic";
import {
  creationOptions,
  IC_DERIVATION_PATH,
  IIConnection,
} from "../../utils/iiConnection";
import { unknownToString } from "../../utils/utils";
import { chooseRecoveryMechanism } from "./chooseRecoveryMechanism";
import { displaySeedPhrase } from "./displaySeedPhrase";

export const setupRecovery = async (
  userNumber: bigint,
  connection: IIConnection
): Promise<void> => {
  const devices = await IIConnection.lookupAll(userNumber);
  const recoveryMechanism = await chooseRecoveryMechanism(devices);
  if (recoveryMechanism === null) {
    return;
  }

  try {
    switch (recoveryMechanism) {
      case "securityKey": {
        const name = "Recovery key";
        let recoverIdentity: WebAuthnIdentity;
        try {
          recoverIdentity = await WebAuthnIdentity.create({
            publicKey: creationOptions(devices, "cross-platform"),
          });
        } catch (err: unknown) {
          await displayError({
            title: "Authentication failure",
            message:
              "Failed to set up a security key as your recovery mechanism. If you don't have an additional security key you can use a seedphrase instead.",
            detail: unknownToString(err, "Unknown error"),
            primaryButton: "Try a different method",
          });
          return setupRecovery(userNumber, connection);
        }

        return await withLoader(() =>
          connection.add(
            userNumber,
            name,
            { cross_platform: null },
            { recovery: null },
            recoverIdentity.getPublicKey().toDer(),
            recoverIdentity.rawId
          )
        );
      }
      case "seedPhrase": {
        const name = "Recovery phrase";
        const seedPhrase = generate().trim();
        const recoverIdentity = await fromMnemonicWithoutValidation(
          seedPhrase,
          IC_DERIVATION_PATH
        );
        await withLoader(() =>
          connection.add(
            userNumber,
            name,
            { seed_phrase: null },
            { recovery: null },
            recoverIdentity.getPublicKey().toDer()
          )
        );
        await displaySeedPhrase(userNumber.toString(10) + " " + seedPhrase);
      }
    }
  } catch (err: unknown) {
    await displayError({
      title: "Failed to set up recovery",
      message: "We failed to set up recovery for this Identity Anchor.",
      detail: unknownToString(err, "Unkwnown error"),
      primaryButton: "Continue",
    });
  }
};
