import type { UnifastPlugin, HastRoot, HastNode } from "@unifast/core";
import { extractLang, extractText, findCodeChild } from "@unifast/core";
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
            const code = findCodeChild(node);
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
