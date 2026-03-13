import { createMemoryHistory, RouterProvider } from "@tanstack/react-router";
import { StrictMode } from "react";
import { renderToStaticMarkup } from "react-dom/server";

import { createRouter } from "./router";

export async function render(url: string): Promise<{ html: string; statusCode: number }> {
  const router = createRouter();
  const memoryHistory = createMemoryHistory({ initialEntries: [url] });
  router.update({ history: memoryHistory });
  await router.load();
  const html = renderToStaticMarkup(
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>,
  );
  const hasNotFound = router.state.matches.some(
    (m) => (m as any).notFoundError !== undefined && (m as any).notFoundError !== null,
  );
  return { html, statusCode: hasNotFound ? 404 : 200 };
}
