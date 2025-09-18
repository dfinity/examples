# Photo gallery

A decentralized photo gallery application built on the Internet Computer blockchain. Users can upload, view, and manage their photos in a decentralized environment.

**WARNING:** This is meant primarily as a demo to show how the response verification library and HTTP gateways can be used to serve images with cache headers. It is not making use of authentication or certification. Use it at your own risk.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/rust/photo_gallery)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```


#### 5. Generate Candid interfaces after making changes to the backend:

```
npm run generate
```

#### 6. Deploy frontend development server (if needed):

```
npm start
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.

