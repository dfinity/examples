module.exports = {
  drivers: {
    chrome: {
      // This version needs to match the chrome version on GitHub Actions
      version: '96.0.4664.45',
      arch: process.arch,
      baseURL: 'https://chromedriver.storage.googleapis.com'
    },
  },
}
