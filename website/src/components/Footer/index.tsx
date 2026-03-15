import { useTranslation, localePath, type LocaleCode } from "~/i18n";

import styles from "./Footer.module.css";

export function Footer({ locale }: { locale?: LocaleCode } = {}) {
  const { locale: resolvedLocale } = useTranslation(locale);

  return (
    <footer className={styles.footer}>
      <div className={styles.inner}>
        <div className={styles.links}>
          <a href={localePath("/docs/introduction/what-is-unifast/", resolvedLocale)}>Docs</a>
          <a href="https://github.com/kenzo-pj/unifast" target="_blank" rel="noopener noreferrer">
            GitHub
          </a>
        </div>
        <span className={styles.credit}>
          Made with ❤️ by{" "}
          <a href="https://github.com/Kenzo-Wada" target="_blank" rel="noopener noreferrer">
            Kenzo Wada
          </a>
        </span>
      </div>
    </footer>
  );
}
