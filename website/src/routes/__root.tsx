import { createRootRoute, Outlet, useRouterState } from "@tanstack/react-router";

import { Layout } from "~/components/Layout";
import { NotFound } from "~/components/NotFound";
import { I18nContext, parseLocaleFromPath, SUPPORTED_LOCALES } from "~/i18n";

export const Route = createRootRoute({
  component: RootComponent,
  notFoundComponent: NotFound,
});

function isHomePage(pathname: string): boolean {
  const clean = pathname.replace(/\/+$/, "") || "/";
  if (clean === "/") return true;
  const segment = clean.slice(1);
  return SUPPORTED_LOCALES.includes(segment as any);
}

function RootComponent() {
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  const { locale } = parseLocaleFromPath(pathname);

  if (isHomePage(pathname)) {
    return (
      <I18nContext.Provider value={{ locale }}>
        <Outlet />
      </I18nContext.Provider>
    );
  }

  return (
    <I18nContext.Provider value={{ locale }}>
      <Layout>
        <Outlet />
      </Layout>
    </I18nContext.Provider>
  );
}
