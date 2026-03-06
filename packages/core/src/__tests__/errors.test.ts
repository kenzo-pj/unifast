import { describe, it, expect } from "vitest";

import { UnifastError, ParseError, CompileError } from "../errors";

describe(UnifastError, () => {
  it("sets name, message, code, and span", () => {
    const err = new UnifastError("msg", "CODE", { start: 0, end: 5 });
    expect(err).toBeInstanceOf(Error);
    expect(err.name).toBe("UnifastError");
    expect(err.message).toBe("msg");
    expect(err.code).toBe("CODE");
    expect(err.span).toStrictEqual({ start: 0, end: 5 });
  });

  it("defaults code and span to undefined", () => {
    const err = new UnifastError("msg");
    expect(err.code).toBeUndefined();
    expect(err.span).toBeUndefined();
  });
});

describe(ParseError, () => {
  it("extends UnifastError with PARSE_ERROR code", () => {
    const err = new ParseError("bad syntax", { start: 10, end: 20 });
    expect(err).toBeInstanceOf(UnifastError);
    expect(err.name).toBe("ParseError");
    expect(err.code).toBe("PARSE_ERROR");
    expect(err.span).toStrictEqual({ start: 10, end: 20 });
  });
});

describe(CompileError, () => {
  it("extends UnifastError with COMPILE_ERROR code", () => {
    const err = new CompileError("compile failed");
    expect(err).toBeInstanceOf(UnifastError);
    expect(err.name).toBe("CompileError");
    expect(err.code).toBe("COMPILE_ERROR");
    expect(err.span).toBeUndefined();
  });
});
