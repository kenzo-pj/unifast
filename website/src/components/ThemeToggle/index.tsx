import { memo } from "react";
import { Toggle } from "@base-ui-components/react/toggle";
import { Sun02Icon, Moon02Icon, ComputerIcon } from "hugeicons-react";
import { useTheme } from "~/hooks/useTheme";
import { useTranslation } from "~/i18n";
import styles from "./ThemeToggle.module.css";

const ICON_SIZE = 16;

export const ThemeToggle = memo(function ThemeToggle() {
  const { t } = useTranslation();
  const { mode, resolved, cycle } = useTheme();
  const label = t(`theme.${mode}` as "theme.light" | "theme.dark" | "theme.system");

  const Icon =
    mode === "light" ? Sun02Icon : mode === "dark" ? Moon02Icon : ComputerIcon;

  return (
    <Toggle
      pressed={resolved === "dark"}
      onPressedChange={cycle}
      className={styles.button}
      aria-label={label}
      title={label}
    >
      <Icon size={ICON_SIZE} />
    </Toggle>
  );
});
