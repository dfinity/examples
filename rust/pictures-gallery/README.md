# Pictures Gallery - Photo Gallery on Internet Computer

A decentralized photo gallery application built on the Internet Computer blockchain. Users can upload, view, and manage their photos in a decentralized environment.

**WARNING** This is meant primarily as a demo to show how the response verification library and http gateways can be used to serve images with cache headers. It is not making use of authentication or certification - use it at your own risk.

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?url=https://github.com/examples/rust/pictures-gallery)


## Features

- **Photo Upload**: Upload images directly to the Internet Computer
- **Photo Gallery**: View all uploaded photos in a responsive grid layout
- **Decentralized Storage**: Photos are stored on-chain using Internet Computer's asset storage

## Architecture

### Backend (Rust)

- **Canister**: `pictures-site-backend`
- **Language**: Rust with ic-cdk
- **Functionality**: Handles photo storage, retrieval, and metadata management

### Frontend (TypeScript/React)

- **Canister**: `pictures-site-frontend`
- **Framework**: React with TypeScript
- **Build Tool**: Vite
- **Styling**: CSS with responsive design

## Getting Started

### Prerequisites

- [DFX SDK](https://internetcomputer.org/docs/building-apps/getting-started/install) installed
- Rust and cargo
- Node.js and npm

### Local Development

1. **Start the Internet Computer replica**:

   ```bash
   dfx start --clean --background
   ```

2. **Deploy the canisters**:

   ```bash
   dfx deploy
   ```

### Development Workflow

- **Generate Candid interfaces** after backend changes:

  ```bash
  npm run generate
  ```

- **Frontend development server** (if needed):
  ```bash
  npm start
  ```

