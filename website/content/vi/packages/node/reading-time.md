---
title: "readingTime()"
description: "Ước lượng thời gian đọc tài liệu và đưa vào kết quả biên dịch."
---

```ts
import { readingTime } from "@unifast/node";
```

## Chữ ký

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Tham số

### options?

Cấu hình cho việc ước lượng thời gian đọc

| Thuộc tính | Kiểu | Mặc định | Mô tả |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Số từ mỗi phút cho văn bản Latin |
| `cjkCharsPerMinute` | `number` | `500` | Số ký tự mỗi phút cho văn bản CJK |

## Giá trị trả về

Plugin bổ sung thuộc tính `readingTime` vào kết quả biên dịch:

| Thuộc tính | Kiểu | Mô tả |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Thời gian đọc ước lượng tính theo phút (tối thiểu 1, làm tròn lên đến 0.5 gần nhất) |
| `result.readingTime.words` | `number` | Tổng số từ (từ Latin + ký tự CJK) |

## Cách dùng

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# My Article

This is a short article with some content that demonstrates
reading time estimation.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

console.log(result.readingTime);
// { minutes: 1, words: 16 }
```

## Ví dụ

### Tùy chỉnh số từ mỗi phút

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // slower reading speed
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### Nội dung CJK

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // adjust for CJK reading speed
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Văn bản hỗn hợp Latin và CJK

Thời gian đọc được tính riêng biệt cho từ Latin và ký tự CJK, sau đó kết hợp lại. Các khối mã được loại khỏi số từ đếm.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Latin words counted at 200 WPM, CJK characters at 500 CPM
console.log(result.readingTime);
```
