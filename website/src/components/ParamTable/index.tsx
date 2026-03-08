import { useState, useCallback } from "react";

import styles from "./ParamTable.module.css";

interface ParamProperty {
  name: string;
  type: string;
  default?: string;
  description?: string;
}

interface Param {
  name: string;
  description?: string;
  type?: string;
  properties?: ParamProperty[];
}

interface ParamTableProps {
  params: Param[];
}

function InfoIcon({ text }: { text: string }) {
  const [open, setOpen] = useState(false);
  const toggle = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setOpen((v) => !v);
  }, []);

  return (
    <button
      type="button"
      className={`${styles.info} ${open ? styles.infoOpen : ""}`}
      onClick={toggle}
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
      aria-label={text}
    >
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <circle cx="8" cy="8" r="7" stroke="currentColor" strokeWidth="1.5" />
        <path d="M8 7v4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
        <circle cx="8" cy="5" r="0.75" fill="currentColor" />
      </svg>
      <span className={styles.tooltip}>{text}</span>
    </button>
  );
}

export function ParamTable({ params }: ParamTableProps) {
  const hasProps = params.some((p) => (p.properties?.length ?? 0) > 0);
  const hasDefault = params.some(
    (p) =>
      p.properties?.some((prop) => prop.default !== null && prop.default !== undefined) ?? false,
  );

  return (
    <div className={styles.wrapper}>
      <table>
        <thead>
          <tr>
            <th>Parameter</th>
            {hasProps && <th>Property</th>}
            <th>Type</th>
            {hasDefault && <th>Default</th>}
          </tr>
        </thead>
        <tbody>
          {params.map((param) => {
            if (!param.properties || param.properties.length === 0) {
              return (
                <tr key={param.name}>
                  <td>
                    <code>{param.name}</code>
                    {param.description && <InfoIcon text={param.description} />}
                  </td>
                  {hasProps && <td />}
                  <td>
                    <code>{param.type}</code>
                  </td>
                  {hasDefault && <td />}
                </tr>
              );
            }

            return param.properties.map((prop, i) => (
              <tr key={`${param.name}-${prop.name}`}>
                {i === 0 && (
                  <td rowSpan={param.properties!.length} className={styles.paramCell}>
                    <code>{param.name}</code>
                    {param.description && <InfoIcon text={param.description} />}
                  </td>
                )}
                <td>
                  <code>{prop.name}</code>
                  {prop.description && <InfoIcon text={prop.description} />}
                </td>
                <td>
                  <code>{prop.type}</code>
                </td>
                {hasDefault && (
                  <td>
                    {prop.default !== null && prop.default !== undefined ? (
                      <code>{prop.default}</code>
                    ) : (
                      "—"
                    )}
                  </td>
                )}
              </tr>
            ));
          })}
        </tbody>
      </table>
    </div>
  );
}
