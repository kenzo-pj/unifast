---
title: "Apa itu unifast?"
description: "unifast adalah compiler Markdown dan MDX berperforma tinggi dengan inti Rust. Pass bawaan untuk GFM, sanitasi, highlighting, dan TOC."
---

unifast adalah compiler Markdown dan MDX berperforma tinggi dengan inti Rust. Ia mencakup use case utama dari remark/rehype dengan mengimplementasikan fitur secara langsung sebagai pass bawaan - bukan melalui kompatibilitas plugin JS.

### Mengapa unifast?

Toolchain Markdown tradisional seperti unified/remark/rehype sangat kuat, tetapi memiliki beberapa trade-off:

- **Overhead performa** - Beberapa transformasi AST JS terakumulasi, terutama dalam skala besar.
- **Koordinasi plugin** - Urutan, kompatibilitas, dan duplikasi di antara puluhan plugin.
- **Tanpa fitur bawaan** - Bahkan tugas dasar seperti GFM atau sanitasi memerlukan paket terpisah.

unifast mengambil pendekatan yang berbeda:

- **Inti Rust** - Parsing, transformasi, dan emisi semuanya berjalan dalam kode native.
- **Pass bawaan** - Fitur umum (GFM, sanitasi, highlighting, TOC) terintegrasi langsung, bukan ditempelkan.
- **Kompilasi tunggal** - Satu panggilan meng-compile Markdown menjadi HTML dengan semua fitur diterapkan.

### Fitur Utama

| Fitur | Deskripsi |
|---------|-------------|
| **CommonMark + GFM** | Tabel, task list, strikethrough, autolink, footnote |
| **Frontmatter** | Ekstraksi metadata YAML, TOML, dan JSON |
| **MDX** | Ekspresi JSX dan import di dalam Markdown |
| **Diagnostics** | Rentang error yang presisi dengan pemetaan baris/kolom |

### Pass Bawaan

Plugin remark/rehype yang umum diimplementasikan ulang sebagai pass Rust native. Tidak perlu npm install, tidak ada kerepotan pengurutan.

| Pass | Deskripsi |
|------|-------------|
| **Sanitization** | Allowlist HTML berbasis schema dengan default yang aman |
| **Syntax Highlighting** | Engine yang dapat dipasang (syntect, Shiki) |
| **Table of Contents** | Pohon heading yang diekstrak secara otomatis |

### Dukungan Platform

unifast berjalan di berbagai platform dari satu inti Rust:

- **`@unifast/node`** - Binding Node.js melalui N-API (napi-rs). Target utama.
- **`@unifast/core`** - Definisi tipe TypeScript yang dibagikan di seluruh paket.
- **`unifast` (CLI)** - Antarmuka command-line untuk skrip dan CI.
- **WASM** - Dukungan runtime browser dan edge (target sekunder).

### Non-goal

unifast **bukan** pengganti drop-in untuk unified. Ia tidak:

- Mengeksekusi plugin JS remark/rehype yang ada di dalam inti.
- Menyediakan kompatibilitas API dengan ekosistem unified.
- Bergantung pada module resolution Node di jalur kompilasi inti.

Sebaliknya, ia menargetkan **kelengkapan use case** - mencakup apa yang dibutuhkan sebagian besar proyek tanpa kompleksitas menyusun pipeline plugin.
