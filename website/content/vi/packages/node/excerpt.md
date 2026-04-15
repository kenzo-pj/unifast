---
title: "excerpt()"
description: "Trích xuất đoạn tóm tắt từ nội dung tài liệu."
---

```ts
import { excerpt } from "@unifast/node";
```

## Chữ ký

```ts
function excerpt(options?: ExcerptPluginOptions): UnifastPlugin
```

## Tham số

### options?

Cấu hình cho việc trích xuất đoạn tóm tắt

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `separator` | `string` | `"<!-- more -->"` | Marker comment phân tách đoạn tóm tắt với phần còn lại |
| `fallbackParagraphs` | `number` | `1` | Số đoạn văn đầu tiên được dùng làm đoạn tóm tắt khi không tìm thấy separator |
| `fallbackCharacters` | `number` | `undefined` | Độ dài tối đa tính theo ký tự cho đoạn tóm tắt fallback (cắt tại ranh giới từ) |

Khi cả `fallbackParagraphs` và `fallbackCharacters` đều được thiết lập, `fallbackParagraphs` sẽ được ưu tiên.

## Giá trị trả về

Plugin bổ sung thuộc tính `excerpt` vào kết quả biên dịch:

| Thuộc tính | Kiểu | Mô tả |
|----------|------|-------------|
| `result.excerpt` | `string \| undefined` | Đoạn tóm tắt văn bản thuần được trích xuất từ tài liệu |

## Cách dùng

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is the introduction to my blog post.

<!-- more -->

The rest of the article continues here with more details.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "This is the introduction to my blog post."
```

## Ví dụ

### Với marker separator

Đặt một comment `<!-- more -->` trong Markdown để đánh dấu rõ ràng nơi đoạn tóm tắt kết thúc:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is a **bold** introduction with some content.

Here is a second paragraph still in the excerpt.

<!-- more -->

This content is not included in the excerpt.
`;

const result = compile(md, {
  plugins: [excerpt()],
});

console.log(result.excerpt);
// "My Blog Post This is a bold introduction with some content. Here is a second paragraph still in the excerpt."
```

### Fallback về đoạn đầu tiên

Khi không tìm thấy marker separator, plugin sẽ fallback sang việc trích xuất N đoạn đầu tiên:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
# My Blog Post

This is the first paragraph of my article.

This is the second paragraph with more details.

This is the third paragraph.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackParagraphs: 2,
    }),
  ],
});

console.log(result.excerpt);
// "This is the first paragraph of my article. This is the second paragraph with more details."
```

### Fallback theo giới hạn ký tự

Cắt đoạn tóm tắt theo độ dài ký tự tối đa, ngắt tại ranh giới từ:

```ts
import { compile, excerpt } from "@unifast/node";

const md = `
This is a long article that goes on and on with lots of content.
`;

const result = compile(md, {
  plugins: [
    excerpt({
      fallbackCharacters: 30,
    }),
  ],
});

console.log(result.excerpt);
// "This is a long article that"
```
