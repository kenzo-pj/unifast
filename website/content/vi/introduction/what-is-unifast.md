---
title: "unifast là gì?"
description: "unifast là trình biên dịch Markdown và MDX hiệu năng cao với lõi Rust. Các pass tích hợp sẵn cho GFM, sanitization, highlighting và TOC."
---

unifast là trình biên dịch Markdown và MDX hiệu năng cao với lõi viết bằng Rust. Nó đáp ứng phần lớn các trường hợp sử dụng phổ biến của remark/rehype bằng cách hiện thực hóa các tính năng trực tiếp như pass tích hợp sẵn - chứ không dựa vào khả năng tương thích với plugin JS.

### Tại sao chọn unifast?

Các toolchain Markdown truyền thống như unified/remark/rehype rất mạnh mẽ nhưng đi kèm với những đánh đổi:

- **Chi phí hiệu năng** - Nhiều phép biến đổi AST trong JS cộng dồn lại, đặc biệt khi quy mô lớn.
- **Phối hợp plugin** - Vấn đề về thứ tự, khả năng tương thích và trùng lặp giữa hàng chục plugin.
- **Không có tính năng tích hợp sẵn** - Ngay cả những tác vụ cơ bản như GFM hay sanitization cũng cần các gói riêng biệt.

unifast lựa chọn một hướng tiếp cận khác:

- **Lõi Rust** - Parsing, biến đổi và xuất ra đều diễn ra trong mã native.
- **Pass tích hợp sẵn** - Các tính năng phổ biến (GFM, sanitization, highlighting, TOC) được tích hợp sẵn, không phải lắp ghép thêm.
- **Biên dịch một lần** - Một lời gọi biên dịch Markdown thành HTML với toàn bộ tính năng được áp dụng.

### Tính năng chính

| Tính năng | Mô tả |
|---------|-------------|
| **CommonMark + GFM** | Bảng, danh sách tác vụ, gạch ngang, autolink, chú thích cuối |
| **Frontmatter** | Trích xuất metadata YAML, TOML và JSON |
| **MDX** | Biểu thức JSX và import trong Markdown |
| **Diagnostics** | Vùng lỗi chính xác kèm ánh xạ dòng/cột |

### Pass tích hợp sẵn

Các plugin remark/rehype phổ biến được viết lại dưới dạng pass Rust native. Không cần npm install, không phải đau đầu vì thứ tự.

| Pass | Mô tả |
|------|-------------|
| **Sanitization** | Danh sách trắng HTML dựa trên schema với mặc định an toàn |
| **Syntax Highlighting** | Engine có thể thay thế (syntect, Shiki) |
| **Table of Contents** | Cây tiêu đề được trích xuất tự động |

### Hỗ trợ nền tảng

unifast chạy trên nhiều nền tảng từ một lõi Rust duy nhất:

- **`@unifast/node`** - Binding Node.js qua N-API (napi-rs). Đây là target chính.
- **`@unifast/core`** - Định nghĩa kiểu TypeScript dùng chung cho toàn bộ các gói.
- **`unifast` (CLI)** - Giao diện dòng lệnh phục vụ script và CI.
- **WASM** - Hỗ trợ trình duyệt và edge runtime (target phụ).

### Những điều unifast không hướng tới

unifast **không** phải là bản thay thế trực tiếp cho unified. Nó không:

- Thực thi các plugin JS của remark/rehype hiện có bên trong lõi.
- Cung cấp API tương thích với hệ sinh thái unified.
- Phụ thuộc vào module resolution của Node trong đường dẫn biên dịch chính.

Thay vào đó, unifast hướng đến **độ hoàn chỉnh theo use-case** - đáp ứng đầy đủ những gì hầu hết dự án cần mà không phải tự lắp ráp pipeline plugin phức tạp.
