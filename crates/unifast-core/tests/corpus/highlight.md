# Syntax Highlighting Corpus

## Rust

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", 42);

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    if let Some(val) = map.get("key") {
        println!("Found: {}", val);
    }
}
```

## JavaScript

```js
const fetchData = async (url) => {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Failed:", error);
    throw error;
  }
};

// Arrow function
const double = (x) => x * 2;
const numbers = [1, 2, 3].map(double);
```

## TypeScript

```ts
interface User {
  name: string;
  age: number;
  email?: string;
}

function greet(user: User): string {
  return `Hello, ${user.name}!`;
}

const users: User[] = [
  { name: "Alice", age: 30 },
  { name: "Bob", age: 25, email: "bob@example.com" },
];
```

## Python

```python
from typing import List, Optional

class DataProcessor:
    def __init__(self, data: List[int]):
        self.data = data

    def process(self, factor: Optional[float] = None) -> List[float]:
        if factor is None:
            factor = 1.0
        return [x * factor for x in self.data]

# Usage
processor = DataProcessor([1, 2, 3, 4, 5])
result = processor.process(2.5)
print(f"Result: {result}")
```

## No Language

```
Plain code block without language specification.
No highlighting should be applied.
```
