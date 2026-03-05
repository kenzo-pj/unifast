import { lazy, Suspense } from "react";
import { Link } from "@tanstack/react-router";
import { ThemeToggle } from "~/components/ThemeToggle";
import { LanguageSwitcher } from "~/components/LanguageSwitcher";
import { GitHubIcon } from "~/components/GitHubIcon";
import { useTranslation, localePath } from "~/i18n";
import styles from "./LandingPage.module.css";

const SearchDialog = lazy(() =>
  import("~/components/SearchDialog").then((m) => ({ default: m.SearchDialog })),
);
const MobileMenu = lazy(() =>
  import("~/components/MobileMenu").then((m) => ({ default: m.MobileMenu })),
);

export function LandingPage() {
  const { locale } = useTranslation();

  return (
    <div className={styles.page}>
      <div className={styles.headerBar}>
        <header className={styles.header}>
          <Link to={localePath("/", locale)} className={styles.logo}>
            unifast
          </Link>
          <div className={styles.headerActions}>
            <Suspense>
              <SearchDialog />
            </Suspense>
            <span className={styles.desktopOnly}>
              <LanguageSwitcher />
            </span>
            <span className={styles.desktopOnly}>
              <ThemeToggle />
            </span>
            <span className={styles.desktopOnly}>
              <a
                href="https://github.com/kenzo-pj/unifast"
                target="_blank"
                rel="noopener noreferrer"
                className={styles.githubLink}
                aria-label="GitHub"
              >
                <GitHubIcon size={20} />
              </a>
            </span>
            <Suspense>
              <MobileMenu />
            </Suspense>
          </div>
        </header>
      </div>

      <section className={styles.hero}>
        <h1 className={styles.title}>unifast</h1>
        <p className={styles.subtitle}>
          High-performance Markdown / MDX compiler built with Rust.
        </p>
        <div className={styles.actions}>
          <Link
            to={localePath("/docs/introduction/what-is-unifast", locale)}
            className={styles.primaryBtn}
          >
            Get Started
          </Link>
          <a
            href="https://github.com/kenzo-pj/unifast"
            target="_blank"
            rel="noopener noreferrer"
            className={styles.secondaryBtn}
          >
            <GitHubIcon size={16} />
            GitHub
          </a>
        </div>
      </section>
    </div>
  );
}
