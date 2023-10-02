import "./index.css";

import App from "./App.tsx";
import { AuthProvider } from "./auth/context/AuthProvider.tsx";
import React from "react";
import ReactDOM from "react-dom/client";
import { RecoilRoot } from "recoil";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RecoilRoot>
      <AuthProvider>
        <App />
      </AuthProvider>
    </RecoilRoot>
  </React.StrictMode>
);
