import { createContext, useContext } from "react";

import en from "./locales/en";
import type { Locale } from "./locales/en";
import ja from "./locales/ja";

export type LocaleCode = "en" | "ja";

export const SUPPORTED_LOCALES: LocaleCode[] = ["en", "ja"];
export const DEFAULT_LOCALE: LocaleCode = "en";

const dictionaries: Record<LocaleCode, Locale> = { en, ja };

interface I18nContextValue {
  locale: LocaleCode;
}

export const I18nContext = createContext<I18nContextValue>({ locale: DEFAULT_LOCALE });

type DotPath<T> = T extends string
  ? ""
  : { [K in keyof T & string]: T[K] extends string ? K : `${K}.${DotPath<T[K]>}` }[keyof T &
      string];

type DotValue<T, P extends string> = P extends `${infer K}.${infer R}`
  ? K extends keyof T
    ? DotValue<T[K], R>
    : never
  : P extends keyof T
    ? T[P]
    : never;

function getByPath<P extends DotPath<Locale>>(dict: Locale, path: P): DotValue<Locale, P> {
  const keys = (path as string).split(".");
  let current: unknown = dict;
  for (const key of keys) {
    current = (current as Record<string, unknown>)[key];
  }
  return current as DotValue<Locale, P>;
}

export function useTranslation() {
  const { locale } = useContext(I18nContext);
  const dict = dictionaries[locale];

  function t<P extends DotPath<Locale>>(path: P): DotValue<Locale, P> {
    return getByPath(dict, path);
  }

  return { t, locale };
}

export function localePath(basePath: string, locale: LocaleCode): string {
  if (locale === DEFAULT_LOCALE) return basePath;
  return `/${locale}${basePath}`;
}

export function parseLocaleFromPath(pathname: string): { locale: LocaleCode; restPath: string } {
  for (const loc of SUPPORTED_LOCALES) {
    if (loc === DEFAULT_LOCALE) continue;
    if (pathname === `/${loc}` || pathname.startsWith(`/${loc}/`)) {
      const rest = pathname.slice(loc.length + 1) || "/";
      return { locale: loc, restPath: rest };
    }
  }
  return { locale: DEFAULT_LOCALE, restPath: pathname };
}
