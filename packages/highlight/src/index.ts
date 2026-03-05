import type { UnifastPlugin, HastRoot, HastNode, HastElement } from "@unifast/core";
import { all, createLowlight } from "lowlight";

export function highlight(): UnifastPlugin {
  const lowlight = createLowlight(all);

  return {
    name: "highlight",
    options: {
      highlight: { enabled: false },
    },
    hastTransform: (hast: HastRoot): HastRoot => {
      return {
        ...hast,
        children: hast.children.map(transformNode),
      };

      function transformNode(node: HastNode): HastNode {
        if (node.type === "element") {
          if (node.tagName === "pre") {
            const code = node.children.find(
              (c): c is HastElement => c.type === "element" && c.tagName === "code",
            );
            if (code) {
              const lang = extractLang(code);
              if (lang && lowlight.registered(lang)) {
                const text = extractText(code);
                const result = lowlight.highlight(lang, text);
                return {
                  ...node,
                  children: [
                    {
                      ...code,
                      properties: {
                        ...code.properties,
                        className: [`language-${lang}`, "hljs"],
                      },
                      children: result.children as HastNode[],
                    },
                  ],
                };
              }
            }
          }
          return {
            ...node,
            children: node.children.map(transformNode),
          };
        }
        if (node.type === "root") {
          return {
            ...node,
            children: node.children.map(transformNode),
          };
        }
        return node;
      }
    },
  };
}

function extractLang(code: HastElement): string | null {
  const classNames = code.properties.className;
  if (!Array.isArray(classNames)) return null;
  for (const cls of classNames) {
    if (typeof cls === "string" && cls.startsWith("language-")) {
      return cls.slice(9);
    }
  }
  return null;
}

function extractText(node: HastNode): string {
  if (node.type === "text") return node.value;
  if (node.type === "element" || node.type === "root") {
    return node.children.map(extractText).join("");
  }
  return "";
}
