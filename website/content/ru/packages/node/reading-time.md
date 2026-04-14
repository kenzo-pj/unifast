---
title: "readingTime()"
description: "Оценивает время чтения документа и добавляет его в результат компиляции."
---

```ts
import { readingTime } from "@unifast/node";
```

## Сигнатура

```ts
function readingTime(options?: ReadingTimePluginOptions): UnifastPlugin
```

## Параметры

### options?

Конфигурация оценки времени чтения

| Свойство | Тип | По умолчанию | Описание |
|----------|------|---------|-------------|
| `wordsPerMinute` | `number` | `200` | Слов в минуту для латинского текста |
| `cjkCharsPerMinute` | `number` | `500` | Символов в минуту для CJK-текста |

## Возвращаемое значение

Плагин добавляет свойство `readingTime` в результат компиляции:

| Свойство | Тип | Описание |
|----------|------|-------------|
| `result.readingTime.minutes` | `number` | Ориентировочное время чтения в минутах (минимум 1, округляется вверх до ближайших 0,5) |
| `result.readingTime.words` | `number` | Общее количество слов (латинские слова + CJK-символы) |

## Использование

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

## Примеры

### Пользовательское значение слов в минуту

```ts
import { compile, readingTime } from "@unifast/node";

const md = `A long article with many words...`;

const result = compile(md, {
  plugins: [
    readingTime({
      wordsPerMinute: 150, // более медленная скорость чтения
    }),
  ],
});

console.log(result.readingTime.minutes);
```

### Содержимое на CJK

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# 日本語の記事

今日は天気がとても良いです。公園で散歩をしました。
`;

const result = compile(md, {
  plugins: [
    readingTime({
      cjkCharsPerMinute: 400, // отрегулировать скорость чтения CJK
    }),
  ],
});

console.log(result.readingTime);
// { minutes: 1, words: ... }
```

### Смешанный латинский и CJK-текст

Время чтения вычисляется отдельно для латинских слов и CJK-символов, а затем суммируется. Блоки кода исключаются из подсчёта слов.

```ts
import { compile, readingTime } from "@unifast/node";

const md = `
# Getting Started ガイド

This guide explains how to use the 設定ファイル for configuration.
`;

const result = compile(md, {
  plugins: [readingTime()],
});

// Латинские слова считаются при 200 слов/мин, CJK-символы — при 500 симв/мин
console.log(result.readingTime);
```
