import "./styles/global.css";
import { PostHogProvider } from "@posthog/react";
import { RouterProvider } from "@tanstack/react-router";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";

import { createRouter } from "./router";

const router = createRouter();

const root = document.querySelector("#root")!;

const posthogKey = import.meta.env.VITE_PUBLIC_POSTHOG_KEY;

const app = (
  <StrictMode>
    {posthogKey ? (
      <PostHogProvider
        apiKey={posthogKey}
        options={{
          api_host: "https://us.i.posthog.com",
          capture_pageview: "history_change",
          capture_pageleave: "if_capture_pageview",
          session_recording: { maskAllInputs: true },
        }}
      >
        <RouterProvider router={router} />
      </PostHogProvider>
    ) : (
      <RouterProvider router={router} />
    )}
  </StrictMode>
);

if (root.innerHTML.trim()) {
  ReactDOM.hydrateRoot(root, app);
} else {
  ReactDOM.createRoot(root).render(app);
}
