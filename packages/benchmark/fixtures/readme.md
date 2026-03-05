# Vortex

A modern, batteries-included web framework for Node.js. Build fast, scalable APIs and
full-stack applications with an intuitive developer experience.

[![npm version](https://img.shields.io/npm/v/vortex)](https://www.npmjs.com/package/vortex)
[![Build Status](https://img.shields.io/github/actions/workflow/status/vortex/vortex/ci.yml)](https://github.com/vortex/vortex/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- **Fast routing** - Radix-tree based router with zero-allocation path matching
- **TypeScript-first** - Full type safety with auto-generated types from route schemas
- **Middleware pipeline** - Composable middleware with async/await support
- **Built-in validation** - Request validation powered by Zod schemas
- **Database integration** - First-class support for PostgreSQL, MySQL, and SQLite
- **Auto documentation** - OpenAPI spec generated from your route definitions
- **Hot reload** - Instant feedback during development with file watching
- **Testing utilities** - Built-in test helpers for integration and unit tests

## Quick Start

### Prerequisites

- Node.js >= 18.0.0
- npm >= 9.0.0 or pnpm >= 8.0.0

### Installation

```bash
npm install vortex
```

Or with pnpm:

```bash
pnpm add vortex
```

### Create Your First App

```js
import { createApp } from "vortex";

const app = createApp();

app.get("/", (ctx) => {
  return ctx.json({ message: "Hello from Vortex!" });
});

app.get("/users/:id", (ctx) => {
  const { id } = ctx.params;
  return ctx.json({ id, name: "Jane Doe" });
});

app.listen(3000, () => {
  console.log("Server running on http://localhost:3000");
});
```

### Project Structure

A typical Vortex project looks like this:

```
my-app/
  src/
    routes/
      index.ts
      users.ts
      posts.ts
    middleware/
      auth.ts
      logging.ts
    models/
      user.ts
      post.ts
    index.ts
  tests/
    routes.test.ts
  package.json
  vortex.config.ts
```

## Configuration

Vortex can be configured through a `vortex.config.ts` file in your project root:

```typescript
import { defineConfig } from "vortex";

export default defineConfig({
  port: 3000,
  host: "0.0.0.0",
  cors: {
    origin: ["http://localhost:5173"],
    methods: ["GET", "POST", "PUT", "DELETE"],
    credentials: true,
  },
  logging: {
    level: "info",
    format: "json",
    destination: "./logs/app.log",
  },
  database: {
    driver: "postgresql",
    url: process.env.DATABASE_URL,
    pool: {
      min: 2,
      max: 10,
    },
  },
  security: {
    rateLimit: {
      windowMs: 15 * 60 * 1000,
      max: 100,
    },
    helmet: true,
  },
});
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `port` | `number` | `3000` | Server port |
| `host` | `string` | `"localhost"` | Server host |
| `cors` | `CorsOptions \| boolean` | `false` | CORS configuration |
| `logging` | `LogConfig` | `{ level: "info" }` | Logging settings |
| `database` | `DatabaseConfig` | `undefined` | Database connection |
| `security` | `SecurityConfig` | `{}` | Security settings |
| `plugins` | `Plugin[]` | `[]` | Plugin list |

## Routing

### Basic Routes

Vortex supports all standard HTTP methods:

```js
app.get("/posts", listPosts);
app.post("/posts", createPost);
app.put("/posts/:id", updatePost);
app.patch("/posts/:id", patchPost);
app.delete("/posts/:id", deletePost);
app.head("/posts/:id", checkPost);
app.options("/posts", postOptions);
```

### Route Groups

Group related routes with a shared prefix and middleware:

```js
const api = app.group("/api/v1");

api.use(authMiddleware);
api.use(rateLimitMiddleware);

api.get("/users", listUsers);
api.get("/users/:id", getUser);
api.post("/users", createUser);
api.put("/users/:id", updateUser);
api.delete("/users/:id", deleteUser);
```

### Route Parameters

Dynamic segments are captured as parameters:

```typescript
app.get("/users/:userId/posts/:postId", (ctx) => {
  const { userId, postId } = ctx.params;
  // Both userId and postId are typed as string
  return ctx.json({ userId, postId });
});
```

### Query Parameters

Access query parameters through `ctx.query`:

```typescript
app.get("/search", (ctx) => {
  const { q, page = "1", limit = "20" } = ctx.query;
  return ctx.json({ query: q, page: Number(page), limit: Number(limit) });
});
```

## Middleware

Middleware functions have access to the context and can modify the request/response pipeline.

### Built-in Middleware

Vortex ships with several built-in middleware functions:

- `cors()` - Cross-Origin Resource Sharing
- `compress()` - Response compression (gzip, brotli)
- `bodyParser()` - Request body parsing (JSON, form data)
- `static()` - Static file serving
- `session()` - Session management
- `helmet()` - Security headers

### Custom Middleware

```js
const logger = async (ctx, next) => {
  const start = Date.now();
  console.log(`--> ${ctx.method} ${ctx.path}`);

  await next();

  const duration = Date.now() - start;
  console.log(`<-- ${ctx.method} ${ctx.path} ${ctx.status} ${duration}ms`);
};

app.use(logger);
```

### Error Handling

```js
app.onError((err, ctx) => {
  console.error("Unhandled error:", err);

  if (err.status === 404) {
    return ctx.json({ error: "Not Found" }, 404);
  }

  return ctx.json({ error: "Internal Server Error" }, 500);
});
```

## Validation

Vortex integrates with Zod for request validation:

```typescript
import { z } from "zod";

const createUserSchema = z.object({
  body: z.object({
    name: z.string().min(1).max(100),
    email: z.string().email(),
    age: z.number().int().min(0).max(150).optional(),
    role: z.enum(["admin", "user", "moderator"]).default("user"),
  }),
  query: z.object({
    notify: z.coerce.boolean().default(false),
  }),
});

app.post("/users", { schema: createUserSchema }, async (ctx) => {
  // ctx.body and ctx.query are fully typed
  const user = await db.users.create(ctx.body);
  return ctx.json(user, 201);
});
```

## Database

### Setup

```bash
npm install @vortex/db
```

```typescript
import { createDatabase } from "@vortex/db";

const db = createDatabase({
  driver: "postgresql",
  url: "postgres://user:pass@localhost:5432/mydb",
});
```

### Queries

```typescript
// Find all
const users = await db.users.findMany({
  where: { active: true },
  orderBy: { createdAt: "desc" },
  limit: 20,
});

// Find one
const user = await db.users.findOne({ id: "user_123" });

// Create
const newUser = await db.users.create({
  name: "Alice",
  email: "alice@example.com",
});

// Update
const updated = await db.users.update(
  { id: "user_123" },
  { name: "Alice Smith" }
);

// Delete
await db.users.delete({ id: "user_123" });
```

### Migrations

```bash
# Create a new migration
npx vortex migrate create add-users-table

# Run pending migrations
npx vortex migrate up

# Rollback last migration
npx vortex migrate down

# Check migration status
npx vortex migrate status
```

## Testing

Vortex provides utilities to test your application:

```typescript
import { createTestClient } from "vortex/test";
import { app } from "../src/index";

const client = createTestClient(app);

describe("User API", () => {
  it("should list users", async () => {
    const res = await client.get("/api/users");
    expect(res.status).toBe(200);
    expect(res.body).toHaveProperty("users");
  });

  it("should create a user", async () => {
    const res = await client.post("/api/users", {
      body: { name: "Test User", email: "test@example.com" },
    });
    expect(res.status).toBe(201);
    expect(res.body.name).toBe("Test User");
  });

  it("should reject invalid input", async () => {
    const res = await client.post("/api/users", {
      body: { name: "" },
    });
    expect(res.status).toBe(400);
    expect(res.body.errors).toBeDefined();
  });
});
```

## Plugins

### Using Plugins

```js
import { createApp } from "vortex";
import { swagger } from "@vortex/swagger";
import { graphql } from "@vortex/graphql";

const app = createApp({
  plugins: [
    swagger({ path: "/docs" }),
    graphql({ path: "/graphql", schema: mySchema }),
  ],
});
```

### Creating Plugins

```typescript
import { definePlugin } from "vortex";

export const myPlugin = definePlugin({
  name: "my-plugin",
  version: "1.0.0",

  setup(app, options) {
    app.use(async (ctx, next) => {
      ctx.set("X-Custom-Header", "plugin-value");
      await next();
    });

    app.get("/plugin-health", (ctx) => {
      return ctx.json({ status: "ok" });
    });
  },
});
```

## Deployment

### Docker

```bash
docker build -t my-vortex-app .
docker run -p 3000:3000 my-vortex-app
```

A recommended `Dockerfile`:

```
FROM node:20-alpine AS builder
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile
COPY . .
RUN pnpm build

FROM node:20-alpine
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./
EXPOSE 3000
CMD ["node", "dist/index.js"]
```

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `PORT` | No | Server port (default: 3000) |
| `HOST` | No | Server host (default: 0.0.0.0) |
| `NODE_ENV` | No | Environment (development, production) |
| `DATABASE_URL` | Yes | Database connection string |
| `SESSION_SECRET` | Yes | Secret for session signing |
| `LOG_LEVEL` | No | Logging level (debug, info, warn, error) |

## Roadmap

The project is under active development. Here is what we are working on:

- [x] Core routing engine
- [x] Middleware pipeline
- [x] Request validation with Zod
- [x] PostgreSQL driver
- [ ] MySQL driver
- [ ] SQLite driver
- [ ] WebSocket support
- [ ] GraphQL integration
- [ ] Server-sent events
- [ ] ~~XML response format~~ (dropped in favor of JSON-only)
- [ ] OpenTelemetry tracing

## Performance

Vortex is designed for high throughput. Here are benchmarks on an M2 MacBook Pro
(Node.js v20, single thread):

| Framework | Requests/sec | Latency (avg) | Latency (p99) |
|-----------|-------------|---------------|----------------|
| Vortex | 48,230 | 0.21ms | 0.89ms |
| Fastify | 42,150 | 0.24ms | 1.12ms |
| Express | 15,420 | 0.65ms | 3.41ms |
| Koa | 28,900 | 0.35ms | 1.58ms |
| Hono | 45,100 | 0.22ms | 0.95ms |

> **Note:** Benchmarks are synthetic and may not reflect real-world performance. Always
> benchmark your own workload. The numbers above were measured using `autocannon` with
> 10 connections over 30 seconds.

## API Reference

### `createApp(options?)`

Creates a new Vortex application instance.

**Parameters:**

- `options.port` - Port number (default: `3000`)
- `options.host` - Host string (default: `"localhost"`)
- `options.plugins` - Array of plugins

**Returns:** `VortexApp`

### `ctx.json(data, status?)`

Sends a JSON response.

**Parameters:**

- `data` - The response body (will be serialized to JSON)
- `status` - HTTP status code (default: `200`)

### `ctx.text(data, status?)`

Sends a plain text response.

### `ctx.redirect(url, status?)`

Redirects the client to the given URL.

### `ctx.params`

Object containing route parameters.

### `ctx.query`

Object containing parsed query string parameters.

### `ctx.body`

Parsed request body (requires body parser middleware).

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
git clone https://github.com/vortex/vortex.git
cd vortex
pnpm install
pnpm dev
```

### Running Tests

```bash
# Run all tests
pnpm test

# Run with coverage
pnpm test:coverage

# Run specific test file
pnpm test -- --filter users
```

## License

MIT - see [LICENSE](LICENSE) for details.

## Acknowledgments

> Vortex draws inspiration from many great projects in the Node.js ecosystem, including
> Express, Fastify, Koa, and Hono. We are grateful to their maintainers and contributors
> for pushing the boundaries of server-side JavaScript.

Special thanks to:

- The [Node.js](https://nodejs.org/) team for the runtime
- The [Zod](https://zod.dev/) team for the validation library
- All our [contributors](https://github.com/vortex/vortex/graphs/contributors)
