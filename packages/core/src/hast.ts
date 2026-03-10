export type HastNode = HastRoot | HastElement | HastText | HastRaw | HastComment | HastDoctype;

export type HastRoot = {
  type: "root";
  children: HastNode[];
};

export type HastElement = {
  type: "element";
  tagName: string;
  properties: Record<string, unknown>;
  children: HastNode[];
};

export type HastText = {
  type: "text";
  value: string;
};

export type HastRaw = {
  type: "raw";
  value: string;
};

export type HastComment = {
  type: "comment";
  value: string;
};

export type HastDoctype = {
  type: "doctype";
};

const VOID_ELEMENTS = new Set([
  "area",
  "base",
  "br",
  "col",
  "embed",
  "hr",
  "img",
  "input",
  "link",
  "meta",
  "param",
  "source",
  "track",
  "wbr",
]);

export function escapeHtml(str: string): string {
  return str
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function serializeProperties(properties: Record<string, unknown>): string {
  const parts: string[] = [];
  const keys = Object.keys(properties).sort();
  for (const key of keys) {
    const value = properties[key];
    if (key === "className" && Array.isArray(value)) {
      if (value.length > 0) {
        parts.push(`class="${escapeHtml(value.join(" "))}"`);
      }
    } else if (key === "class" && typeof value === "string") {
      parts.push(`class="${escapeHtml(value)}"`);
    } else if (value === true) {
      parts.push(key);
    } else if (value !== false && value != null) {
      parts.push(`${key}="${escapeHtml(String(value))}"`);
    }
  }
  return parts.length > 0 ? " " + parts.join(" ") : "";
}

function serializeNode(node: HastNode): string {
  switch (node.type) {
    case "root": {
      return node.children.map(serializeNode).join("");
    }
    case "element": {
      const props = serializeProperties(node.properties);
      if (VOID_ELEMENTS.has(node.tagName)) {
        return `<${node.tagName}${props} />`;
      }
      const children = node.children.map(serializeNode).join("");
      return `<${node.tagName}${props}>${children}</${node.tagName}>`;
    }
    case "text": {
      return escapeHtml(node.value);
    }
    case "raw": {
      return node.value;
    }
    case "comment": {
      return `<!--${node.value}-->`;
    }
    case "doctype": {
      return "<!DOCTYPE html>";
    }
    default: {
      return "";
    }
  }
}

// Public utility for external consumers (shiki, user code).
// The internal compile() path in @unifast/node uses native.stringifyHast()
// which delegates to Rust's emit/html/stringify.rs instead.
export function hastToHtml(hast: HastRoot): string {
  return hast.children.map(serializeNode).join("");
}
