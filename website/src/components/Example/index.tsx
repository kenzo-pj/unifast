import { useEffect, useMemo, useRef, useState } from "react";
import { createRoot, type Root } from "react-dom/client";

import { CopyButton } from "~/components/CopyButton";
import { MarkdownIcon, TypeScriptIcon } from "~/components/icons/logos";

import styles from "./Example.module.css";

function TabIcon({ label }: { label: string }) {
  if (label.endsWith(".ts") || label.endsWith(".tsx") || label.endsWith(".js")) {
    return <TypeScriptIcon size={14} className={styles.tabIcon} />;
  }
  return <MarkdownIcon size={14} className={styles.tabIcon} />;
}

interface CodeTab {
  label: string;
  html: string;
}

interface ExampleData {
  html: string;
  codeHtml: string;
  codes?: CodeTab[];
}

interface ExampleProps {
  data: ExampleData;
  renderMath?: boolean;
}

export function Example({ data, renderMath }: ExampleProps) {
  const codes = useMemo(
    () => data.codes ?? [{ label: "md", html: data.codeHtml }],
    [data.codes, data.codeHtml],
  );
  const [activeTab, setActiveTab] = useState(0);
  const previewRef = useRef<HTMLDivElement>(null);
  const codeRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!renderMath || !previewRef.current) return;
    const el = previewRef.current;
    const inlineEls = el.querySelectorAll<HTMLElement>("code.math-inline");
    const displayEls = el.querySelectorAll<HTMLElement>("pre.math-display");
    if (inlineEls.length === 0 && displayEls.length === 0) return;

    void Promise.all([import("katex"), import("katex/dist/katex.min.css")]).then(
      ([{ default: katex }]) => {
        for (const mathEl of inlineEls) {
          const tex = mathEl.textContent ?? "";
          const wrapper = document.createElement("span");
          katex.render(tex, wrapper, { displayMode: false, throwOnError: false });
          mathEl.replaceWith(wrapper);
        }
        for (const preEl of displayEls) {
          const tex = preEl.textContent ?? "";
          const wrapper = document.createElement("div");
          katex.render(tex, wrapper, { displayMode: true, throwOnError: false });
          preEl.replaceWith(wrapper);
        }
      },
    );
  }, [renderMath, data.html]);

  useEffect(() => {
    const container = codeRef.current;
    if (!container) return;

    const pres = container.querySelectorAll("pre");
    const roots: Root[] = [];

    pres.forEach((pre) => {
      const code = pre.querySelector("code");
      const text = (code || pre).textContent || "";
      const wrapper = document.createElement("div");
      wrapper.className = styles.codeBlock;
      pre.parentNode!.insertBefore(wrapper, pre);
      wrapper.append(pre);
      const btnWrapper = document.createElement("span");
      wrapper.append(btnWrapper);
      const root = createRoot(btnWrapper);
      root.render(<CopyButton text={text} />);
      roots.push(root);
    });

    return () => roots.forEach((root) => setTimeout(() => root.unmount(), 0));
  }, [activeTab, codes]);

  const activeCode = codes[activeTab] ?? codes[0];
  const showTabs = codes.length > 1;

  return (
    <div className={styles.card}>
      <div
        ref={previewRef}
        className={styles.preview}
        dangerouslySetInnerHTML={{ __html: data.html }}
      />
      {showTabs && (
        <div className={styles.tabBar}>
          {codes.map((tab, i) => (
            <button
              key={tab.label}
              className={`${styles.tab} ${i === activeTab ? styles.tabActive : ""}`}
              onClick={() => setActiveTab(i)}
            >
              <TabIcon label={tab.label} />
              {tab.label}
            </button>
          ))}
        </div>
      )}
      <div
        ref={codeRef}
        className={showTabs ? styles.codeWrapperTabbed : styles.codeWrapper}
        dangerouslySetInnerHTML={{ __html: activeCode.html }}
      />
    </div>
  );
}
