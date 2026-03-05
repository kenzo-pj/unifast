---
title: "sectionize()"
description: "Wrap heading groups in `<section>` elements."
---

```ts
import { sectionize } from "@unifast/node";
```

### Signature

```ts
function sectionize(): UnifastPlugin
```

### Parameters

None.

### Returns

`UnifastPlugin`

## Usage

### Basic usage

```ts
import { compile, sectionize } from "@unifast/node";

const md = `# Introduction

Some introductory text.

## Getting Started

Instructions for getting started.

## Configuration

Configuration details here.`;

const result = compile(md, {
  plugins: [sectionize()],
});
// Each heading and its following content are wrapped in a <section> element
```
