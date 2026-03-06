import type { HastNode, HastElement } from "./hast";

export function extractLang(code: HastElement): string | null {
  const classNames = code.properties.className;
  if (!Array.isArray(classNames)) return null;
  for (const cls of classNames) {
    if (typeof cls === "string" && cls.startsWith("language-")) {
      return cls.slice(9);
    }
  }
  return null;
}

export function extractText(node: HastNode): string {
  if (node.type === "text") return node.value;
  if (node.type === "element" || node.type === "root") {
    return node.children.map(extractText).join("");
  }
  return "";
}

export function findCodeChild(element: HastElement): HastElement | undefined {
  return element.children.find(
    (child): child is HastElement => child.type === "element" && child.tagName === "code",
  );
}

export function visitHast(node: HastNode, visitor: (node: HastNode) => HastNode | void): HastNode {
  const result = visitor(node);
  const current = result ?? node;
  if (current.type === "element" || current.type === "root") {
    return {
      ...current,
      children: current.children.map((child) => visitHast(child, visitor)),
    };
  }
  return current;
}
