import { Tabs } from "@base-ui/react/tabs";
import { ComputerTerminal01Icon } from "hugeicons-react";
import { memo, useState, useCallback, useMemo } from "react";

import { CopyButton } from "~/components/CopyButton";

import styles from "./PackageInstall.module.css";

const STORAGE_KEY = "unifast-pkg-manager";

const MANAGERS = [
  { id: "npm", label: "npm", command: (pkg: string) => `npm install ${pkg}` },
  { id: "yarn", label: "yarn", command: (pkg: string) => `yarn add ${pkg}` },
  { id: "pnpm", label: "pnpm", command: (pkg: string) => `pnpm add ${pkg}` },
  { id: "bun", label: "bun", command: (pkg: string) => `bun add ${pkg}` },
] as const;

function getDefaultManager(): string {
  if (typeof window === "undefined") return "npm";
  return localStorage.getItem(STORAGE_KEY) ?? "npm";
}

// highlighted HTML is pre-sanitized by the Rust sanitize pass at build time
const ManagerPanel = memo(function ManagerPanel({
  id,
  command,
  highlighted,
}: {
  id: string;
  command: string;
  highlighted?: string;
}) {
  return (
    <Tabs.Panel value={id} className={styles.panel}>
      <code>
        <span className={styles.prompt}>$</span>
        {highlighted ? <span dangerouslySetInnerHTML={{ __html: highlighted }} /> : command}
      </code>
    </Tabs.Panel>
  );
});

interface PackageInstallProps {
  package: string;
  highlighted?: Record<string, string>;
}

export const PackageInstall = memo(function PackageInstall(props: PackageInstallProps) {
  const pkg = props.package;
  const highlighted = props.highlighted;
  const [manager, setManager] = useState(getDefaultManager);

  const currentCommand =
    MANAGERS.find((m) => m.id === manager)?.command(pkg) ?? `npm install ${pkg}`;

  const handleTabChange = useCallback((value: string | number | null) => {
    if (typeof value === "string") {
      setManager(value);
      localStorage.setItem(STORAGE_KEY, value);
    }
  }, []);

  const panels = useMemo(
    () =>
      MANAGERS.map((m) => (
        <ManagerPanel
          key={m.id}
          id={m.id}
          command={m.command(pkg)}
          highlighted={highlighted?.[m.id]}
        />
      )),
    [pkg, highlighted],
  );

  return (
    <Tabs.Root value={manager} onValueChange={handleTabChange}>
      <div className={styles.container}>
        <Tabs.List className={styles.tabList}>
          <ComputerTerminal01Icon size={14} className={styles.terminalIcon} />
          {MANAGERS.map((m) => (
            <Tabs.Tab key={m.id} value={m.id} className={styles.tab}>
              {m.label}
            </Tabs.Tab>
          ))}
        </Tabs.List>

        <div className={styles.codeArea}>
          {panels}
          <CopyButton text={currentCommand} />
        </div>
      </div>
    </Tabs.Root>
  );
});
