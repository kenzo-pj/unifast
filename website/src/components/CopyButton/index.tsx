import { Tooltip } from "@base-ui/react/tooltip";
import { Copy01Icon, Tick01Icon } from "hugeicons-react";
import { memo, useState, useCallback } from "react";

import styles from "./CopyButton.module.css";

interface CopyButtonProps {
  text: string;
}

export const CopyButton = memo(function CopyButton({ text }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);
  const [hovering, setHovering] = useState(false);

  const handleCopy = useCallback(async () => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  }, [text]);

  return (
    <Tooltip.Provider delay={200}>
      <Tooltip.Root open={copied || hovering} onOpenChange={setHovering}>
        <Tooltip.Trigger className={styles.button} onClick={handleCopy} type="button">
          {copied ? <Tick01Icon size={14} /> : <Copy01Icon size={14} />}
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Positioner sideOffset={6}>
            <Tooltip.Popup className={styles.tooltip}>{copied ? "Copied!" : "Copy"}</Tooltip.Popup>
          </Tooltip.Positioner>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
});
