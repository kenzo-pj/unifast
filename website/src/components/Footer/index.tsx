import { Link } from "@tanstack/react-router";

import { useTranslation, localePath } from "~/i18n";

import styles from "./Footer.module.css";

export function Footer() {
  const { locale } = useTranslation();

  return (
    <footer className={styles.footer}>
      <div className={styles.inner}>
        <div className={styles.links}>
          <Link to={localePath("/docs/introduction/what-is-unifast", locale)}>Docs</Link>
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
