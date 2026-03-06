import { useEffect, useRef, useState, type ReactNode } from "react";

import { CopyButton } from "~/components/CopyButton";

import styles from "./DocContent.module.css";

interface CodeBlockProps {
  __rawCode?: string;
  dangerouslySetInnerHTML?: { __html: string };
  children?: ReactNode;
  className?: string;
}

function CodeBlock({ __rawCode, dangerouslySetInnerHTML, children, ...rest }: CodeBlockProps) {
  const preRef = useRef<HTMLPreElement>(null);
  const [copyText, setCopyText] = useState(__rawCode ?? "");

  useEffect(() => {
    if (!__rawCode && preRef.current) {
      setCopyText(preRef.current.textContent ?? "");
    }
  }, [__rawCode, children]);

  return (
    <div className={styles.codeBlock}>
      {dangerouslySetInnerHTML ? (
        <pre ref={preRef} dangerouslySetInnerHTML={dangerouslySetInnerHTML} {...rest} />
      ) : (
        <pre ref={preRef} {...rest}>
          {children}
        </pre>
      )}
      <CopyButton text={copyText} />
    </div>
  );
}

interface ScrollTableProps {
  children?: ReactNode;
  [key: string]: unknown;
}

function ScrollTable({ children, ...rest }: ScrollTableProps) {
  return (
    <div className={styles.tableWrapper}>
      <table {...rest}>{children}</table>
    </div>
  );
}

export const mdxComponents = {
  pre: CodeBlock,
  table: ScrollTable,
};
