---
title: "Temel Kavramlar"
description: "unifast'in çok aşamalı derleme ardışık düzenini, dahili geçişlerini ve MdAst ile HAst dönüşümlerinin nasıl çalıştığını öğrenin."
---

unifast, Markdown'u çok aşamalı bir ardışık düzen aracılığıyla derler. Bu aşamaları anlamak, derleyiciyi yapılandırmanıza ve doğru plugin'leri seçmenize yardımcı olur.

### Derleme Ardışık Düzeni

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Her aşama, belgeyi bir ara temsil (IR) aracılığıyla dönüştürür.

### Ara Temsiller

| IR | İsim | Amaç |
|----|------|------|
| **IR0** | MdAst | Markdown yapısı - başlıklar, paragraflar, listeler, kod blokları |
| **IR1** | HAst | HTML yapısı - elemanlar, öznitelikler, metin düğümleri |
| **IR2** | JsAst | Yalnızca MDX - ESM import'ları ve JSX ifadeleri |

Parser, ardından HTML yayımı için **HAst**'e indirgenen **MdAst** üretir. MDX girdileri ek olarak JavaScript çıktısı için **JsAst** düğümleri üretir.

### Geçişler

Geçişler, belirli aşamalarda AST'ye uygulanan dönüşümlerdir. unifast, yaygın görevler için dahili geçişlere sahiptir:

**MdAst geçişleri** (HTML'e indirgemeden önce):

- **Normalize** - Sonraki geçişler için tutarlı yapı
- **Slug** - Metin içeriğinden başlık ID'leri üretir
- **TOC** - Başlıklardan içindekiler tablosunu çıkarır
- **Tanım Çözümleme** - Bağlantı/görsel referans tanımlarını çözer

**HAst geçişleri** (HTML'e indirgedikten sonra):

- **Sanitize** - İzin verilmeyen HTML elemanlarını ve özniteliklerini kaldırır
- **Highlight** - Kod bloklarına sözdizimi vurgulaması uygular
- **Cleanup** - Gereksiz düğümleri ve boşlukları kaldırır

Geçişler aşamaya göre sıralanır; çalıştırma sırası hakkında endişelenmeniz gerekmez.

### Plugin'ler

Plugin'ler, dahili geçişleri yapılandıran TypeScript paketleridir. Derleme sırasında keyfi JavaScript çalıştırmazlar; bunun yerine Rust çekirdeğinin belgenizi nasıl işleyeceğini kontrol eden seçenekleri ayarlarlar.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Her plugin, Rust çekirdeği çalışmadan önce derleme seçeneklerine birleştirilen bir yapılandırma nesnesi döndürür. Bu, sıcak yolu tamamen native kodda tutar.

### Çıktı Biçimleri

Derleyici, `outputKind` seçeneği aracılığıyla birden fazla çıktı biçimini destekler:

| Biçim | Açıklama |
|-------|----------|
| `"html"` | HTML string (varsayılan) |
| `"hast"` | HAst JSON - özel render için HTML AST |
| `"mdast"` | MdAst JSON - analiz için Markdown AST |
| `"mdx-js"` | JavaScript modül string'i (yalnızca MDX) |

### Tanılamalar

Derleyici, sorunları sonuçtaki `diagnostics` dizisi aracılığıyla raporlar. Her tanılama, kesin hata konumu için bir önem düzeyi, mesaj ve isteğe bağlı kaynak aralığı içerir.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
