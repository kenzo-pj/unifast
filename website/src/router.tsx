import { createRouter as createTanStackRouter } from "@tanstack/react-router";

import { routeTree } from "./routeTree.gen";

export function createRouter() {
  const base = import.meta.env.BASE_URL.replace(/\/+$/, "");
  return createTanStackRouter({
    routeTree,
    basepath: base || "/",
    trailingSlash: "always",
    context: { head: "" },
    defaultPreload: "intent",
  });
}

declare module "@tanstack/react-router" {
  interface Register {
    router: ReturnType<typeof createRouter>;
  }
}
