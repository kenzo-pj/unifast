import { memo, useState, useEffect, useRef } from "react";
import styles from "./TableOfContents.module.css";

interface TocEntry {
  depth: number;
  text: string;
  slug: string;
}

const TocItem = memo(function TocItem({
  entry,
  isActive,
}: {
  entry: TocEntry;
  isActive: boolean;
}) {
  return (
    <li className={styles.item}>
      <a
        href={`#${entry.slug}`}
        className={`${styles.link}${entry.depth >= 3 ? ` ${styles.depth3}` : ""}${isActive ? ` ${styles.linkActive}` : ""}`}
      >
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

    const slugs = toc.map((e) => e.slug);
    const headings = slugs
      .map((s) => document.getElementById(s))
      .filter(Boolean) as HTMLElement[];

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
        {toc.map((entry) => (
          <TocItem
            key={entry.slug}
            entry={entry}
            isActive={activeSlug === entry.slug}
          />
        ))}
      </ul>
    </nav>
  );
});
