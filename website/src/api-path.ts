import { SUPPORTED_LOCALES, DEFAULT_LOCALE } from "./i18n";

export function pageRouteToApiPath(route: string): string {
  const trimmed = route.replace(/\/$/, "");
  if (trimmed === "") return "/api/index.json";
  for (const loc of SUPPORTED_LOCALES) {
    if (loc === DEFAULT_LOCALE) continue;
    if (trimmed === `/${loc}`) return `/api/${loc}/index.json`;
  }
  return `/api${trimmed}.json`;
}
