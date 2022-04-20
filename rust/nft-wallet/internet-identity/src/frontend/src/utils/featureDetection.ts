export const hasRequiredFeatures = async (url: URL): Promise<boolean> => {
  if (window.PublicKeyCredential === undefined) return false;
  if (url.hash === "#compatibilityNotice") return false;
  // For mobile devices we want to make sure we can use platform authenticators
  if (!navigator.userAgent.match(/(iPhone|iPod|iPad|Android)/)) return true;
  try {
    return await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
  } catch (error) {
    return false;
  }
};
