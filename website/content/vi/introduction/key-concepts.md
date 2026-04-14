---
title: "Các khái niệm cốt lõi"
description: "Tìm hiểu về pipeline biên dịch đa tầng của unifast, các pass tích hợp sẵn, và cách các phép biến đổi MdAst và HAst hoạt động."
---

unifast biên dịch Markdown thông qua một pipeline đa tầng. Việc hiểu rõ các giai đoạn này sẽ giúp bạn cấu hình trình biên dịch và lựa chọn đúng plugin phù hợp.

### Pipeline biên dịch

```
Input text
  → Parse (Markdown or MDX)
  → IR0: MdAst (Markdown structure)
  → Normalize + Built-in MdAst passes
  → Lower to IR1: HAst (HTML structure)
  → HAst passes (sanitize, highlight, etc.)
  → Emit (HTML string)
```

Mỗi giai đoạn biến đổi tài liệu thông qua một biểu diễn trung gian (intermediate representation - IR).

### Các biểu diễn trung gian

| IR | Tên | Mục đích |
|----|------|---------|
| **IR0** | MdAst | Cấu trúc Markdown - tiêu đề, đoạn văn, danh sách, khối mã |
| **IR1** | HAst | Cấu trúc HTML - phần tử, thuộc tính, nút văn bản |
| **IR2** | JsAst | Chỉ dành cho MDX - các import ESM và biểu thức JSX |

Parser sinh ra **MdAst**, sau đó được hạ xuống thành **HAst** để xuất ra HTML. Với đầu vào MDX, trình biên dịch còn sinh thêm các nút **JsAst** cho phần đầu ra JavaScript.

### Passes

Pass là các phép biến đổi được áp dụng lên AST ở những giai đoạn cụ thể. unifast tích hợp sẵn các pass cho những tác vụ phổ biến:

**MdAst passes** (trước khi hạ xuống HTML):

- **Normalize** - Chuẩn hóa cấu trúc để phục vụ các pass phía sau
- **Slug** - Sinh ID cho tiêu đề từ nội dung văn bản
- **TOC** - Trích xuất mục lục từ các tiêu đề
- **Definition Resolution** - Phân giải các định nghĩa tham chiếu link/image

**HAst passes** (sau khi hạ xuống HTML):

- **Sanitize** - Loại bỏ các phần tử và thuộc tính HTML không được phép
- **Highlight** - Áp dụng syntax highlighting cho khối mã
- **Cleanup** - Loại bỏ các nút và khoảng trắng không cần thiết

Các pass được sắp xếp theo phase - bạn không cần lo lắng về thứ tự thực thi.

### Plugins

Plugin là các gói TypeScript dùng để cấu hình các pass tích hợp sẵn. Chúng không chạy JavaScript tùy ý trong quá trình biên dịch - thay vào đó, chúng thiết lập các tùy chọn để điều khiển cách lõi Rust xử lý tài liệu của bạn.

```ts
import { compile, gfm, sanitize } from "@unifast/node";

const result = compile(source, {
  plugins: [gfm(), sanitize()],
});
```

Mỗi plugin trả về một đối tượng cấu hình được gộp vào các tùy chọn biên dịch trước khi lõi Rust chạy. Nhờ vậy, đường dẫn xử lý chính (hot path) hoàn toàn nằm trong mã native.

### Định dạng đầu ra

Trình biên dịch hỗ trợ nhiều định dạng đầu ra thông qua tùy chọn `outputKind`:

| Định dạng | Mô tả |
|--------|-------------|
| `"html"` | Chuỗi HTML (mặc định) |
| `"hast"` | HAst JSON - AST của HTML để render tùy biến |
| `"mdast"` | MdAst JSON - AST của Markdown để phân tích |
| `"mdx-js"` | Chuỗi JavaScript module (chỉ dành cho MDX) |

### Diagnostics

Trình biên dịch báo cáo các vấn đề thông qua mảng `diagnostics` trong kết quả. Mỗi diagnostic bao gồm mức độ nghiêm trọng, thông điệp, và vùng nguồn tùy chọn để xác định chính xác vị trí lỗi.

```ts
const result = compile(source);

for (const d of result.diagnostics) {
  console.warn(`[${d.level}] ${d.message}`);
}
```
