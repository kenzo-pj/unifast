import type { HastNode, HastRoot } from "@unifast/core";
import StyleToObject from "style-to-object";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type CreateElement = (type: any, props: any, ...children: any[]) => any;

export type ComponentMap = Record<string, unknown>;

export type HastToReactOptions = {
  createElement: CreateElement;
  Fragment: unknown;
  components?: ComponentMap;
};

const PROP_RENAMES: Record<string, string> = {
  class: "className",
  for: "htmlFor",
  accesskey: "accessKey",
  autocapitalize: "autoCapitalize",
  autocomplete: "autoComplete",
  autofocus: "autoFocus",
  autoplay: "autoPlay",
  charset: "charSet",
  colspan: "colSpan",
  contenteditable: "contentEditable",
  crossorigin: "crossOrigin",
  datetime: "dateTime",
  enctype: "encType",
  formaction: "formAction",
  formenctype: "formEncType",
  formmethod: "formMethod",
  formnovalidate: "formNoValidate",
  formtarget: "formTarget",
  hreflang: "hrefLang",
  htmlfor: "htmlFor",
  httpequiv: "httpEquiv",
  inputmode: "inputMode",
  maxlength: "maxLength",
  mediagroup: "mediaGroup",
  minlength: "minLength",
  nomodule: "noModule",
  novalidate: "noValidate",
  readonly: "readOnly",
  referrerpolicy: "referrerPolicy",
  rowspan: "rowSpan",
  spellcheck: "spellCheck",
  srcdoc: "srcDoc",
  srclang: "srcLang",
  srcset: "srcSet",
  tabindex: "tabIndex",
  usemap: "useMap",
};

function parseStyleString(style: string): Record<string, string> {
  const result: Record<string, string> = {};
  StyleToObject(style, (name, value) => {
    if (name && value) {
      const camelName = name.startsWith("--")
        ? name
        : name.replaceAll(/-([a-z])/g, (_, c) => String(c).toUpperCase());
      result[camelName] = value;
    }
  });
  return result;
}

function convertProperties(properties: Record<string, unknown>): Record<string, unknown> {
  const props: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(properties)) {
    if (value === false || value == null) continue;

    if (key === "style" && typeof value === "string") {
      props.style = parseStyleString(value);
      continue;
    }

    if (key === "className" && Array.isArray(value)) {
      props.className = value.join(" ");
      continue;
    }

    const renamed = PROP_RENAMES[key];
    if (renamed) {
      props[renamed] = value;
    } else {
      props[key] = value;
    }
  }
  return props;
}

function convertNode(node: HastNode, options: HastToReactOptions, key: number): unknown {
  const { createElement, Fragment, components } = options;

  switch (node.type) {
    case "root": {
      const children = convertChildren(node.children, options);
      return createElement(Fragment, null, ...children);
    }
    case "element": {
      const component = components?.[node.tagName] ?? node.tagName;
      const props = convertProperties(node.properties);
      props.key = key;
      const children = convertChildren(node.children, options);
      if (children.length > 0) {
        return createElement(component, props, ...children);
      }
      return createElement(component, props);
    }
    case "text": {
      return node.value;
    }
    case "raw": {
      return createElement("div", {
        key,
        dangerouslySetInnerHTML: { __html: node.value },
      });
    }
    case "comment":
    case "doctype": {
      return null;
    }
    default: {
      return null;
    }
  }
}

function convertChildren(children: HastNode[], options: HastToReactOptions): unknown[] {
  const result: unknown[] = [];
  for (let i = 0; i < children.length; i++) {
    const converted = convertNode(children[i], options, i);
    if (converted != null) {
      result.push(converted);
    }
  }
  return result;
}

export function hastToReact(hast: HastRoot, options: HastToReactOptions): unknown {
  return convertNode(hast, options, 0);
}
