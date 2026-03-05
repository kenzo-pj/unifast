import { renderToString } from "react-dom/server";
import {
  createMemoryHistory,
  RouterProvider,
} from "@tanstack/react-router";
import { createRouter } from "./router";

export async function render(url: string): Promise<{ html: string; statusCode: number }> {
  const router = createRouter();
  const memoryHistory = createMemoryHistory({ initialEntries: [url] });
  router.update({ history: memoryHistory });
  await router.load();
  const html = renderToString(<RouterProvider router={router} />);
  const hasNotFound = router.state.matches.some((m) => m.notFoundError != null);
  return { html, statusCode: hasNotFound ? 404 : 200 };
}
