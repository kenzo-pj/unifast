export class UnifastError extends Error {
  constructor(
    message: string,
    public code?: string,
    public span?: { start: number; end: number },
  ) {
    super(message);
    this.name = "UnifastError";
  }
}

export class ParseError extends UnifastError {
  constructor(message: string, span?: { start: number; end: number }) {
    super(message, "PARSE_ERROR", span);
    this.name = "ParseError";
  }
}

export class CompileError extends UnifastError {
  constructor(message: string) {
    super(message, "COMPILE_ERROR");
    this.name = "CompileError";
  }
}
