import { memo, type ReactNode } from "react";

import { GitHubIcon } from "~/components/GitHubIcon";
import { LanguageSwitcher } from "~/components/LanguageSwitcher";
import { MobileMenu } from "~/components/MobileMenu";
import { SearchDialog } from "~/components/SearchDialog";
import { Sidebar } from "~/components/Sidebar";
import { ThemeToggle } from "~/components/ThemeToggle";
import { useTranslation, localePath, type LocaleCode } from "~/i18n";

import styles from "./Layout.module.css";

const Header = memo(function Header({ locale }: { locale: LocaleCode }) {
  useTranslation(locale);

  return (
    <div className={styles.headerBar}>
      <header className={styles.header}>
        <a href={localePath("/", locale)} className={styles.logo}>
          unifast
        </a>
        <div className={styles.headerActions}>
          <SearchDialog />
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
          <MobileMenu />
        </div>
      </header>
    </div>
  );
});

interface LayoutProps {
  children: ReactNode;
  locale: LocaleCode;
  pathname?: string;
}

export function Layout({ children, locale, pathname }: LayoutProps) {
  const resolvedPathname = pathname ?? globalThis.location?.pathname ?? "/";
  return (
    <div className={styles.layout}>
      <Header locale={locale} />
      <div className={styles.body}>
        <aside className={styles.sidebar}>
          <Sidebar hideLogo locale={locale} pathname={resolvedPathname} />
        </aside>
        <main className={styles.main}>{children}</main>
      </div>
    </div>
  );
}
