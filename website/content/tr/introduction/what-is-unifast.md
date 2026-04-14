---
title: "unifast nedir?"
description: "unifast, Rust çekirdeğine sahip yüksek performanslı bir Markdown ve MDX derleyicisidir. GFM, temizleme, vurgulama ve TOC için dahili geçişlere sahiptir."
---

unifast, Rust çekirdeğine sahip yüksek performanslı bir Markdown ve MDX derleyicisidir. Özellikleri JS plugin uyumluluğu yerine doğrudan dahili geçişler olarak uygulayarak remark/rehype'ın yaygın kullanım senaryolarını kapsar.

### Neden unifast?

unified/remark/rehype gibi geleneksel Markdown araç zincirleri güçlüdür ancak bazı ödünleşimleri vardır:

- **Performans yükü** - Birden fazla JS AST dönüşümü, özellikle büyük ölçekte birikir.
- **Plugin koordinasyonu** - Onlarca plugin arasında sıralama, uyumluluk ve tekrar sorunları.
- **Dahili özelliklerin eksikliği** - GFM veya temizleme gibi temel görevler bile ayrı paketler gerektirir.

unifast farklı bir yaklaşım benimser:

- **Rust çekirdeği** - Ayrıştırma, dönüştürme ve yayma işlemlerinin tamamı native kodda gerçekleşir.
- **Dahili geçişler** - Yaygın özellikler (GFM, temizleme, vurgulama, TOC) sonradan eklenmiş değil dahilidir.
- **Tek derleme** - Tek bir çağrı, tüm özellikler uygulanmış halde Markdown'u HTML'e derler.

### Temel Özellikler

| Özellik | Açıklama |
|---------|----------|
| **CommonMark + GFM** | Tablolar, görev listeleri, üstü çizili metin, otomatik bağlantılar, dipnotlar |
| **Frontmatter** | YAML, TOML ve JSON üst veri çıkarma |
| **MDX** | Markdown içinde JSX ifadeleri ve import'lar |
| **Tanılamalar** | Satır/sütun eşlemesiyle kesin hata konumları |

### Dahili Geçişler

Yaygın remark/rehype plugin'leri native Rust geçişleri olarak yeniden uygulanmıştır. npm install yok, sıralama zorlukları yok.

| Geçiş | Açıklama |
|-------|----------|
| **Temizleme** | Güvenli varsayılanlarla şema tabanlı HTML izin listesi |
| **Sözdizimi Vurgulama** | Takılabilir motorlar (syntect, Shiki) |
| **İçindekiler Tablosu** | Otomatik çıkarılan başlık ağacı |

### Platform Desteği

unifast, tek bir Rust çekirdeğinden birden fazla platformda çalışır:

- **`@unifast/node`** - N-API (napi-rs) aracılığıyla Node.js bağlayıcısı. Birincil hedef.
- **`@unifast/core`** - Tüm paketler arasında paylaşılan TypeScript tip tanımları.
- **`unifast` (CLI)** - Scriptler ve CI için komut satırı arayüzü.
- **WASM** - Tarayıcı ve edge runtime desteği (ikincil hedef).

### Hedef Olmayanlar

unifast, unified için **sorunsuz bir yedek değildir**. Şunları yapmaz:

- Çekirdek içinde mevcut remark/rehype JS plugin'lerini çalıştırmaz.
- unified ekosistemiyle API uyumluluğu sağlamaz.
- Çekirdek derleme yolunda Node'un modül çözümlemesine bağımlı olmaz.

Bunun yerine, bir plugin ardışık düzeni kurma karmaşıklığı olmadan çoğu projenin ihtiyaç duyduğu şeyleri kapsayarak **kullanım senaryosu bütünlüğünü** hedefler.
