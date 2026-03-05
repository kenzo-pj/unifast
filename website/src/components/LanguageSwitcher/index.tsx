import { Select } from "@base-ui/react/select";
import { useNavigate, useRouterState } from "@tanstack/react-router";
import { LanguageSquareIcon } from "hugeicons-react";
import { useCallback } from "react";

import { useTranslation, DEFAULT_LOCALE, SUPPORTED_LOCALES, parseLocaleFromPath } from "~/i18n";
import type { LocaleCode } from "~/i18n";

import styles from "./LanguageSwitcher.module.css";

const LOCALE_ITEMS = SUPPORTED_LOCALES.map((loc) => ({
  value: loc,
  label: loc === "en" ? "English" : "日本語",
}));

export function LanguageSwitcher() {
  const { locale } = useTranslation();
  const pathname = useRouterState({ select: (s) => s.location.pathname });
  const navigate = useNavigate();
  const { restPath } = parseLocaleFromPath(pathname);

  const handleValueChange = useCallback(
    (nextLocale: LocaleCode | null) => {
      if (!nextLocale || nextLocale === locale) return;
      const targetPath = nextLocale === DEFAULT_LOCALE ? restPath : `/${nextLocale}${restPath}`;
      navigate({ to: targetPath });
    },
    [locale, restPath, navigate],
  );

  return (
    <Select.Root value={locale} onValueChange={handleValueChange} items={LOCALE_ITEMS}>
      <Select.Trigger className={styles.trigger}>
        <LanguageSquareIcon size={16} />
        <Select.Value className={styles.value} />
        <Select.Icon className={styles.icon}>
          <ChevronIcon />
        </Select.Icon>
      </Select.Trigger>

      <Select.Portal>
        <Select.Positioner className={styles.positioner} sideOffset={8}>
          <Select.Popup className={styles.popup}>
            <Select.List className={styles.list}>
              {LOCALE_ITEMS.map((item) => (
                <Select.Item key={item.value} value={item.value} className={styles.item}>
                  <Select.ItemText>{item.label}</Select.ItemText>
                  <Select.ItemIndicator className={styles.indicator}>
                    <CheckIcon />
                  </Select.ItemIndicator>
                </Select.Item>
              ))}
            </Select.List>
          </Select.Popup>
        </Select.Positioner>
      </Select.Portal>
    </Select.Root>
  );
}

function ChevronIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M2 3.5L5 6.5L8 3.5" stroke="currentColor" strokeWidth="1.5" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M2.5 6L5 8.5L9.5 3.5" stroke="currentColor" strokeWidth="1.5" />
    </svg>
  );
}
