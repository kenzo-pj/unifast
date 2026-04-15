import { memo, useState, useEffect, useRef } from "react";

import styles from "./TableOfContents.module.css";

interface TocEntry {
  depth: number;
  text: string;
  slug: string;
}

const TocItem = memo(function TocItem({ entry, isActive }: { entry: TocEntry; isActive: boolean }) {
  const className = `${styles.link}${entry.depth >= 3 ? ` ${styles.depth3}` : ""}${isActive ? ` ${styles.linkActive}` : ""}`;
  if (!entry.slug) {
    return (
      <li className={styles.item}>
        <span className={className}>{entry.text}</span>
      </li>
    );
  }
  return (
    <li className={styles.item}>
      <a href={`#${entry.slug}`} className={className}>
        {entry.text}
      </a>
    </li>
  );
});

interface TableOfContentsProps {
  toc: TocEntry[];
}

export const TableOfContents = memo(function TableOfContents({ toc }: TableOfContentsProps) {
  const [activeSlug, setActiveSlug] = useState("");
  const observerRef = useRef<IntersectionObserver | null>(null);

  useEffect(() => {
    if (toc.length === 0) return;

    const headings: HTMLElement[] = [];
    for (const entry of toc) {
      if (!entry.slug) continue;
      const el = document.querySelector(`#${CSS.escape(entry.slug)}`);
      if (el instanceof HTMLElement) headings.push(el);
    }

    if (headings.length === 0) return;

    observerRef.current = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveSlug(entry.target.id);
            break;
          }
        }
      },
      { rootMargin: "-80px 0px -60% 0px", threshold: 0 },
    );

    for (const h of headings) {
      observerRef.current.observe(h);
    }

    return () => observerRef.current?.disconnect();
  }, [toc]);

  if (toc.length === 0) return null;

  return (
    <nav className={styles.nav} aria-label="Table of contents">
      <div className={styles.title}>On this page</div>
      <ul className={styles.list}>
        {toc.map((entry, i) => (
          <TocItem
            key={`${i}-${entry.slug}`}
            entry={entry}
            isActive={Boolean(entry.slug) && activeSlug === entry.slug}
          />
        ))}
      </ul>
    </nav>
  );
});
