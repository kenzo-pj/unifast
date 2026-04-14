---
title: "Konsep Kunci"
description: "Pahami pipeline kompilasi multi-tahap unifast, pass bawaan, dan cara kerja transformasi MdAst serta HAst."
---

unifast meng-compile Markdown melalui pipeline multi-tahap. Memahami tahapan ini akan membantu Anda mengonfigurasi compiler dan memilih plugin yang tepat.

### Pipeline Kompilasi

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Setiap tahap mentransformasikan dokumen melalui intermediate representation (IR).

### Intermediate Representation

| IR | Nama | Tujuan |
|----|------|---------|
| **IR0** | MdAst | Struktur Markdown - heading, paragraf, list, code block |
| **IR1** | HAst | Struktur HTML - elemen, atribut, text node |
| **IR2** | JsAst | MDX saja - import ESM dan ekspresi JSX |

Parser menghasilkan **MdAst**, yang kemudian diturunkan menjadi **HAst** untuk emisi HTML. Input MDX juga menghasilkan node **JsAst** untuk output JavaScript.

### Pass

Pass adalah transformasi yang diterapkan pada AST pada fase tertentu. unifast memiliki pass bawaan untuk tugas umum:

**Pass MdAst** (sebelum penurunan ke HTML):

- **Normalize** - Struktur yang konsisten untuk pass berikutnya
- **Slug** - Menghasilkan ID heading dari konten teks
- **TOC** - Mengekstrak daftar isi dari heading
- **Definition Resolution** - Menyelesaikan definisi referensi link/gambar

**Pass HAst** (setelah penurunan ke HTML):

- **Sanitize** - Menghapus elemen dan atribut HTML yang tidak diizinkan
- **Highlight** - Menerapkan syntax highlighting pada code block
- **Cleanup** - Menghapus node dan whitespace yang tidak perlu

Pass diurutkan berdasarkan fase - Anda tidak perlu khawatir tentang urutan eksekusi.

### Plugin

Plugin adalah paket TypeScript yang mengonfigurasi pass bawaan. Plugin tidak menjalankan JavaScript sembarang selama kompilasi - sebaliknya, plugin menetapkan opsi yang mengontrol cara inti Rust memproses dokumen Anda.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Setiap plugin mengembalikan sebuah objek konfigurasi yang digabungkan ke dalam opsi compile sebelum inti Rust berjalan. Hal ini menjaga hot path sepenuhnya dalam kode native.

### Format Output

Compiler mendukung beberapa format output melalui opsi `outputKind`:

| Format | Deskripsi |
|--------|-------------|
| `"html"` | String HTML (default) |
| `"hast"` | JSON HAst - AST HTML untuk rendering kustom |
| `"mdast"` | JSON MdAst - AST Markdown untuk analisis |
| `"mdx-js"` | String modul JavaScript (MDX saja) |

### Diagnostics

Compiler melaporkan masalah melalui array `diagnostics` pada hasil. Setiap diagnostic mencakup tingkat severity, pesan, dan span sumber opsional untuk lokasi error yang presisi.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
