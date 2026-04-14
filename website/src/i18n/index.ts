import { createContext, useContext } from "react";

import de from "./locales/de";
import en, { type Locale } from "./locales/en";
import es from "./locales/es";
import fr from "./locales/fr";
import hi from "./locales/hi";
import id from "./locales/id";
import it from "./locales/it";
import ja from "./locales/ja";
import ko from "./locales/ko";
import ptBR from "./locales/pt-BR";
import ru from "./locales/ru";
import tr from "./locales/tr";
import vi from "./locales/vi";
import zhCN from "./locales/zh-CN";
import zhTW from "./locales/zh-TW";

export type LocaleCode =
  | "en"
  | "ja"
  | "zh-CN"
  | "zh-TW"
  | "ko"
  | "fr"
  | "it"
  | "es"
  | "pt-BR"
  | "de"
  | "ru"
  | "hi"
  | "id"
  | "tr"
  | "vi";

export const SUPPORTED_LOCALES: LocaleCode[] = [
  "en",
  "ja",
  "zh-CN",
  "zh-TW",
  "ko",
  "fr",
  "it",
  "es",
  "pt-BR",
  "de",
  "ru",
  "hi",
  "id",
  "tr",
  "vi",
];

export const DEFAULT_LOCALE: LocaleCode = "en";

export const LOCALE_LABELS: Record<LocaleCode, string> = {
  en: "English",
  ja: "日本語",
  "zh-CN": "简体中文",
  "zh-TW": "繁體中文",
  ko: "한국어",
  fr: "Français",
  it: "Italiano",
  es: "Español",
  "pt-BR": "Português (Brasil)",
  de: "Deutsch",
  ru: "Русский",
  hi: "हिन्दी",
  id: "Bahasa Indonesia",
  tr: "Türkçe",
  vi: "Tiếng Việt",
};

const dictionaries: Record<LocaleCode, Locale> = {
  en,
  ja,
  "zh-CN": zhCN,
  "zh-TW": zhTW,
  ko,
  fr,
  it,
  es,
  "pt-BR": ptBR,
  de,
  ru,
  hi,
  id,
  tr,
  vi,
};

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

export function getTranslation(locale: LocaleCode) {
  const dict = dictionaries[locale];

  function t<P extends DotPath<Locale>>(path: P): DotValue<Locale, P> {
    return getByPath(dict, path);
  }

  return { t, locale };
}

export function useTranslation(localeProp?: LocaleCode) {
  const contextLocale = useContext(I18nContext).locale;
  return getTranslation(localeProp ?? contextLocale);
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
