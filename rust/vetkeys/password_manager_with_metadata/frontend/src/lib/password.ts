import type { Principal } from "@icp-sdk/core/principal";
import type { PasswordMetadata } from "../declarations/password_manager_with_metadata/backend.did";

export interface PasswordModel {
    owner: Principal;
    parentVaultName: string;
    passwordName: string;
    content: string;
    metadata: PasswordMetadata;
}

export function passwordFromContent(
    owner: Principal,
    parentVaultName: string,
    passwordName: string,
    content: string,
    metadata: PasswordMetadata,
): PasswordModel {
    return {
        owner,
        parentVaultName,
        passwordName,
        content,
        metadata,
    };
}

export function summarize(password: PasswordModel, maxLength = 50) {
    const text = password.content.replace(/<[^>]+>/, "");
    return text.slice(0, maxLength) + (text.length > maxLength ? "..." : "");
}
