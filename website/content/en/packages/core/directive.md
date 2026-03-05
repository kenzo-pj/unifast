---
title: "directive()"
description: "Support container directives with `:::name` syntax."
---

```ts
import { directive } from "@unifast/node";
```

### Signature

```ts
function directive(): UnifastPlugin
```

### Parameters

None.

### Returns

`UnifastPlugin`

## Examples

### Input

```md
:::note
This is a note container. Use it to highlight important information.
:::

:::warning title="Deprecation Notice"
This API will be removed in the next major version.
:::
```

### Output

```html
<div class="directive directive-note" data-directive="note">
  <p>This is a note container. Use it to highlight important information.</p>
</div>

<div class="directive directive-warning" data-directive="warning">
  <p>This API will be removed in the next major version.</p>
</div>
```

You can style these with CSS using the `.directive-note`, `.directive-warning` classes, similar to GitHub Alerts but with fully custom names.

## Usage

```ts
import { compile, directive } from "@unifast/node";

const result = compile(md, { plugins: [directive()] });
```

### Containers with attributes

```md
:::warning title="Deprecation Notice"
This API will be removed in the next major version.
:::
```

### Nested content

Directives support full Markdown inside, including lists, code blocks, and inline formatting.

```md
:::note
You can include **bold**, *italic*, and other Markdown inside containers.

- List items work too
- As does `inline code`
:::
```
