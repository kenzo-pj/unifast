import { Dialog } from "@base-ui/react/dialog";
import { useNavigate } from "@tanstack/react-router";
import { Search01Icon } from "hugeicons-react";
import { useState, useEffect, useCallback } from "react";

import { useSearch } from "~/hooks/useSearch";
import { useTranslation } from "~/i18n";

import styles from "./SearchDialog.module.css";

export function SearchDialog() {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const { query, results, loading, search, reset } = useSearch();
  const [activeIndex, setActiveIndex] = useState(-1);
  const navigate = useNavigate();

  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        setOpen(true);
      }
    }
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

  const handleOpenChange = useCallback(
    (nextOpen: boolean) => {
      setOpen(nextOpen);
      if (!nextOpen) {
        reset();
        setActiveIndex(-1);
      }
    },
    [reset],
  );

  const goToResult = useCallback(
    (url: string) => {
      setOpen(false);
      reset();
      setActiveIndex(-1);
      navigate({ to: url });
    },
    [navigate, reset],
  );

  const handleInputKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setActiveIndex((i) => (i < results.length - 1 ? i + 1 : 0));
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setActiveIndex((i) => (i > 0 ? i - 1 : results.length - 1));
      } else if (e.key === "Enter" && activeIndex >= 0 && results[activeIndex]) {
        e.preventDefault();
        goToResult(results[activeIndex].url);
      }
    },
    [results, activeIndex, goToResult],
  );

  useEffect(() => {
    setActiveIndex(results.length > 0 ? 0 : -1);
  }, [results]);

  const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/.test(navigator.userAgent);

  return (
    <Dialog.Root open={open} onOpenChange={handleOpenChange}>
      <Dialog.Trigger className={styles.trigger}>
        <Search01Icon size={14} />
        {t("search.trigger")}
        <kbd className={styles.kbd}>{isMac ? "\u2318K" : "Ctrl+K"}</kbd>
      </Dialog.Trigger>

      <Dialog.Portal>
        <Dialog.Backdrop className={styles.backdrop} />
        <Dialog.Popup className={styles.popup}>
          <div className={styles.inputWrapper}>
            <Search01Icon size={16} className={styles.inputIcon} />
            <input
              className={styles.input}
              type="text"
              placeholder={t("search.placeholder")}
              value={query}
              onChange={(e) => search(e.target.value)}
              onKeyDown={handleInputKeyDown}
              autoFocus
            />
          </div>

          {query && !loading && results.length === 0 && (
            <div className={styles.empty}>
              {t("search.noResults")} &ldquo;{query}&rdquo;
            </div>
          )}

          {results.length > 0 && (
            <ul className={styles.resultList}>
              {results.map((result, i) => (
                <li
                  key={result.url}
                  className={styles.resultItem}
                  data-active={i === activeIndex ? "" : undefined}
                  onMouseEnter={() => setActiveIndex(i)}
                  onClick={() => goToResult(result.url)}
                >
                  <div className={styles.resultTitle}>{result.title}</div>
                  <div
                    className={styles.resultExcerpt}
                    dangerouslySetInnerHTML={{ __html: result.excerpt }}
                  />
                </li>
              ))}
            </ul>
          )}

          <div className={styles.footer}>
            <span className={styles.footerHint}>
              <kbd className={styles.footerKbd}>{"\u2191\u2193"}</kbd> {t("search.navigate")}
            </span>
            <span className={styles.footerHint}>
              <kbd className={styles.footerKbd}>{"\u21B5"}</kbd> {t("search.open")}
            </span>
            <span className={styles.footerHint}>
              <kbd className={styles.footerKbd}>esc</kbd> {t("search.close")}
            </span>
          </div>
        </Dialog.Popup>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
