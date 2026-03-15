import { ArrowLeft01Icon, ArrowRight01Icon, PencilEdit01Icon } from "hugeicons-react";
import { useCallback, useEffect, useRef, type ComponentType } from "react";
import { createRoot, type Root } from "react-dom/client";

import { CopyButton } from "~/components/CopyButton";
import { TableOfContents } from "~/components/TableOfContents";
import { useTranslation, DEFAULT_LOCALE, type LocaleCode } from "~/i18n";

import type { TranslationStatus } from "../../../plugins/vite-plugin-translation-status";
import { mdxComponents } from "./mdxComponents";

import styles from "./DocContent.module.css";

interface PageLink {
  label: string;
  href: string;
}

interface DocContentProps {
  html?: string;
  MdxContent?: ComponentType<{ components?: Record<string, ComponentType> }>;
  frontmatter: Record<string, unknown>;
  toc: Array<{ depth: number; text: string; slug: string }>;
  translationStatus?: TranslationStatus;
  slug?: string;
  prevPage?: PageLink;
  nextPage?: PageLink;
  locale?: LocaleCode;
}

export function DocContent({
  html,
  MdxContent,
  frontmatter,
  toc,
  translationStatus,
  slug,
  prevPage,
  nextPage,
  locale: localeProp,
}: DocContentProps) {
  const { t, locale } = useTranslation(localeProp);
  const title = frontmatter.title as string | undefined;
  const description = frontmatter.description as string | undefined;
  const handleContentClick = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    const target = e.target as HTMLElement;
    const anchor = target.closest("a[href]");
    if (anchor) {
      const href = anchor.getAttribute("href");
      if (href && href.startsWith("/") && !anchor.getAttribute("target")) {
        e.preventDefault();
        globalThis.location.href = href;
        return;
      }
    }
    const heading = target.closest("h1[id], h2[id], h3[id]");
    if (heading) {
      const id = heading.getAttribute("id")!;
      history.replaceState(null, "", `#${id}`);
      heading.scrollIntoView({ behavior: "smooth" });
    }
  }, []);

  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const container = contentRef.current;
    if (!container || !html) return;

    container.querySelectorAll("table").forEach((table) => {
      if (table.parentElement?.classList.contains(styles.tableWrapper)) return;
      const wrapper = document.createElement("div");
      wrapper.className = styles.tableWrapper;
      table.parentNode!.insertBefore(wrapper, table);
      wrapper.append(table);
    });

    const pres = container.querySelectorAll("pre");
    const roots: Root[] = [];

    pres.forEach((pre) => {
      const code = pre.querySelector("code");
      const text = (code || pre).textContent || "";
      const outer = document.createElement("div");
      outer.className = styles.codeBlock;
      pre.parentNode!.insertBefore(outer, pre);
      outer.append(pre);
      const btnWrapper = document.createElement("span");
      outer.append(btnWrapper);
      const root = createRoot(btnWrapper);
      root.render(<CopyButton text={text} />);
      roots.push(root);
    });

    return () => roots.forEach((root) => setTimeout(() => root.unmount(), 0));
  }, [html]);

  const showBanner =
    locale !== DEFAULT_LOCALE && translationStatus && translationStatus !== "translated";

  return (
    <div className={styles.page}>
      <article className={styles.wrapper} data-pagefind-body>
        {showBanner && (
          <div className={`${styles.banner} ${styles[`banner_${translationStatus}`]}`}>
            <span>
              {translationStatus === "missing"
                ? t("i18n.untranslatedBanner")
                : t("i18n.outdatedBanner")}
            </span>
          </div>
        )}
        {title && (
          <h1 className={`${styles.title}${description ? ` ${styles.titleWithDescription}` : ""}`}>
            {title}
          </h1>
        )}
        {description && <p className={styles.description}>{description}</p>}
        {MdxContent ? (
          <div key={slug} className={styles.content} onClick={handleContentClick}>
            <MdxContent components={mdxComponents} />
          </div>
        ) : html ? (
          <div
            key={slug}
            ref={contentRef}
            className={styles.content}
            onClick={handleContentClick}
            dangerouslySetInnerHTML={{ __html: html }}
          />
        ) : null}
        <footer className={styles.footer}>
          {slug && (
            <a
              className={styles.editLink}
              href={`https://github.com/kenzo-pj/unifast/edit/main/website/content/en/${slug}${MdxContent ? ".mdx" : ".md"}`}
              target="_blank"
              rel="noopener noreferrer"
            >
              <PencilEdit01Icon size={16} />
              {t("nav.editThisPage")}
            </a>
          )}
          {(prevPage || nextPage) && (
            <nav className={styles.pageNav}>
              {prevPage ? (
                <a href={prevPage.href} className={styles.pageNavCard}>
                  <span className={styles.pageNavDirection}>{t("nav.previous")}</span>
                  <span className={styles.pageNavTitle}>
                    <ArrowLeft01Icon size={16} className={styles.pageNavArrow} />
                    {prevPage.label}
                  </span>
                </a>
              ) : (
                <span />
              )}
              {nextPage ? (
                <a
                  href={nextPage.href}
                  className={`${styles.pageNavCard} ${styles.pageNavCardNext}`}
                >
                  <span className={styles.pageNavDirection}>{t("nav.next")}</span>
                  <span className={styles.pageNavTitle}>
                    {nextPage.label}
                    <ArrowRight01Icon size={16} className={styles.pageNavArrow} />
                  </span>
                </a>
              ) : (
                <span />
              )}
            </nav>
          )}
          <p className={styles.attribution}>
            Made with ❤️ by{" "}
            <a href="https://github.com/Kenzo-Wada" target="_blank" rel="noopener noreferrer">
              Kenzo Wada
            </a>
          </p>
        </footer>
      </article>
      <TableOfContents toc={toc} />
    </div>
  );
}
