import { Dialog } from "@base-ui/react/dialog";
import { Menu01Icon, Cancel01Icon } from "hugeicons-react";
import { useState, useCallback, useEffect, useRef } from "react";

import { GitHubIcon } from "~/components/GitHubIcon";
import { LanguageSwitcher } from "~/components/LanguageSwitcher";
import { Sidebar } from "~/components/Sidebar";
import { ThemeToggle } from "~/components/ThemeToggle";
import type { LocaleCode } from "~/i18n";

import styles from "./MobileMenu.module.css";

export function MobileMenu({
  pathname: pathnameProp,
  locale: _locale,
}: {
  pathname?: string;
  locale?: LocaleCode;
} = {}) {
  const [open, setOpen] = useState(false);
  const pathname = pathnameProp ?? globalThis.location?.pathname ?? "";
  const pathnameRef = useRef(pathname);

  useEffect(() => {
    if (pathnameRef.current !== pathname) {
      pathnameRef.current = pathname;
      setOpen(false);
    }
  }, [pathname]);

  const handleOpenChange = useCallback((nextOpen: boolean) => setOpen(nextOpen), []);

  return (
    <Dialog.Root open={open} onOpenChange={handleOpenChange}>
      <Dialog.Trigger className={styles.menuButton} aria-label="Menu">
        <Menu01Icon size={20} />
      </Dialog.Trigger>

      <Dialog.Portal>
        <Dialog.Backdrop className={styles.backdrop} />
        <Dialog.Popup className={styles.sheet}>
          <div className={styles.sheetHeader}>
            <LanguageSwitcher />
            <ThemeToggle />
            <a
              href="https://github.com/kenzo-pj/unifast"
              target="_blank"
              rel="noopener noreferrer"
              className={styles.githubLink}
              aria-label="GitHub"
            >
              <GitHubIcon size={18} />
            </a>
            <Dialog.Close className={styles.closeButton}>
              <Cancel01Icon size={18} />
            </Dialog.Close>
          </div>

          <div className={styles.nav}>
            <Sidebar hideLogo />
          </div>
        </Dialog.Popup>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
