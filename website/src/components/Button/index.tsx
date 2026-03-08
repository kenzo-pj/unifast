import { Button as BaseButton } from "@base-ui/react/button";
import { memo, type ComponentPropsWithoutRef, type ReactElement } from "react";

import styles from "./Button.module.css";

interface ButtonProps extends Omit<ComponentPropsWithoutRef<"button">, "className"> {
  render?: ReactElement<Record<string, unknown>>;
  children: React.ReactNode;
}

export const Button = memo(function Button({ render, children, ...rest }: ButtonProps) {
  return (
    <BaseButton className={styles.button} render={render} {...rest}>
      {children}
    </BaseButton>
  );
});
