import { Tabs } from "@base-ui/react/tabs";
import { memo, useState, useEffect, useCallback, useMemo, type ComponentType } from "react";

import { CopyButton } from "~/components/CopyButton";
import { NpmIcon, YarnIcon, PnpmIcon, BunIcon } from "~/components/icons/logos";

import styles from "./PackageInstall.module.css";

const STORAGE_KEY = "unifast-pkg-manager";

const MANAGERS = [
  { id: "npm", label: "npm", icon: NpmIcon, command: (pkg: string) => `npm install ${pkg}` },
  { id: "yarn", label: "yarn", icon: YarnIcon, command: (pkg: string) => `yarn add ${pkg}` },
  { id: "pnpm", label: "pnpm", icon: PnpmIcon, command: (pkg: string) => `pnpm add ${pkg}` },
  { id: "bun", label: "bun", icon: BunIcon, command: (pkg: string) => `bun add ${pkg}` },
] as const satisfies readonly {
  id: string;
  label: string;
  icon: ComponentType<{ size?: number; className?: string }>;
  command: (pkg: string) => string;
}[];

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
  const { highlighted } = props;
  const [manager, setManager] = useState("npm");

  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) setManager(stored);
  }, []);

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
          {MANAGERS.map((m) => (
            <Tabs.Tab key={m.id} value={m.id} className={styles.tab}>
              <m.icon size={14} className={styles.tabIcon} />
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
