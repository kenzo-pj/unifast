# Meridian SDK Reference

Meridian is a comprehensive cloud platform SDK for building distributed applications. This
reference covers the complete API surface, including authentication, data storage, real-time
messaging, file management, and serverless functions.

## Table of Contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Database](#database)
- [Real-time](#real-time)
- [Storage](#storage)
- [Functions](#functions)
- [Edge Config](#edge-config)
- [CLI Reference](#cli-reference)
- [Error Handling](#error-handling)
- [TypeScript Support](#typescript-support)
- [Migration Guide](#migration-guide)

## Installation

### Node.js

Install the SDK using your preferred package manager:

```bash
npm install @meridian/sdk
```

Or with pnpm:

```bash
pnpm add @meridian/sdk
```

Or with yarn:

```bash
yarn add @meridian/sdk
```

### Browser

For browser environments, you can use the CDN:

```html
<script type="module">
  import { createClient } from "https://cdn.meridian.dev/sdk@3/index.mjs";
</script>
```

### Deno

```typescript
import { createClient } from "https://deno.land/x/meridian/mod.ts";
```

### Requirements

| Platform | Minimum Version |
|----------|----------------|
| Node.js | 18.0.0 |
| Deno | 1.35.0 |
| Bun | 1.0.0 |
| Chrome | 90+ |
| Firefox | 88+ |
| Safari | 15+ |
| Edge | 90+ |

## Authentication

Meridian supports multiple authentication strategies. All methods return a session object
that is automatically managed by the SDK.

### Initialize the Client

```typescript
import { createClient } from "@meridian/sdk";

const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  region: "us-east-1",
});
```

### Email and Password

The most common authentication method for web applications:

```typescript
// Sign up
const { user, session, error } = await meridian.auth.signUp({
  email: "user@example.com",
  password: "securePassword123!",
  metadata: {
    firstName: "Jane",
    lastName: "Doe",
  },
});

if (error) {
  console.error("Sign up failed:", error.message);
}

// Sign in
const { user, session, error } = await meridian.auth.signIn({
  email: "user@example.com",
  password: "securePassword123!",
});

// Sign out
await meridian.auth.signOut();

// Sign out all sessions
await meridian.auth.signOut({ scope: "global" });
```

### OAuth Providers

Meridian supports the following OAuth providers:

- Google
- GitHub
- Apple
- Microsoft
- Discord
- Slack
- Twitter

```typescript
// Redirect-based OAuth
const { url, error } = await meridian.auth.signInWithOAuth({
  provider: "github",
  redirectTo: "https://myapp.com/auth/callback",
  scopes: ["user:email", "read:org"],
});

// Handle callback
const { session, error } = await meridian.auth.handleOAuthCallback({
  code: urlParams.get("code"),
  state: urlParams.get("state"),
});
```

### Magic Links

Passwordless authentication via email:

```typescript
// Send magic link
const { error } = await meridian.auth.signInWithMagicLink({
  email: "user@example.com",
  redirectTo: "https://myapp.com/dashboard",
});

// Verify the token from the magic link URL
const { session, error } = await meridian.auth.verifyMagicLink({
  token: urlParams.get("token"),
});
```

### Phone Authentication

SMS-based one-time password:

```typescript
// Send OTP
const { error } = await meridian.auth.signInWithPhone({
  phone: "+15551234567",
});

// Verify OTP
const { session, error } = await meridian.auth.verifyPhone({
  phone: "+15551234567",
  code: "123456",
});
```

### Session Management

```typescript
// Get current session
const session = await meridian.auth.getSession();

// Refresh session
const { session, error } = await meridian.auth.refreshSession();

// Listen for auth state changes
meridian.auth.onAuthStateChange((event, session) => {
  switch (event) {
    case "SIGNED_IN":
      console.log("User signed in:", session.user.email);
      break;
    case "SIGNED_OUT":
      console.log("User signed out");
      break;
    case "TOKEN_REFRESHED":
      console.log("Token refreshed");
      break;
    case "SESSION_EXPIRED":
      console.log("Session expired");
      break;
  }
});
```

### Multi-Factor Authentication

```typescript
// Enable MFA for current user
const { secret, qrCode, error } = await meridian.auth.mfa.enroll({
  method: "totp",
  issuer: "MyApp",
});

// Verify MFA during sign-in
const { session, error } = await meridian.auth.mfa.verify({
  factorId: "factor_abc123",
  code: "123456",
});

// List enrolled factors
const { factors, error } = await meridian.auth.mfa.listFactors();

// Unenroll a factor
await meridian.auth.mfa.unenroll({ factorId: "factor_abc123" });
```

### Auth Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `autoRefreshToken` | `boolean` | `true` | Automatically refresh expiring tokens |
| `persistSession` | `boolean` | `true` | Persist session to storage |
| `storageKey` | `string` | `"meridian-auth"` | Key used for session storage |
| `flowType` | `"implicit" \| "pkce"` | `"pkce"` | OAuth flow type |
| `detectSessionInUrl` | `boolean` | `true` | Check URL for auth params on init |

## Database

Meridian provides a real-time, Postgres-compatible database with a powerful query builder.
All queries support both promise-based and real-time subscription patterns.

### Table Operations

#### Select

Retrieve rows from a table with filtering, ordering, and pagination:

```typescript
// Basic select
const { data, error } = await meridian.db
  .from("posts")
  .select("*");

// Select specific columns
const { data, error } = await meridian.db
  .from("posts")
  .select("id, title, created_at");

// Select with relations (joins)
const { data, error } = await meridian.db
  .from("posts")
  .select(`
    id,
    title,
    content,
    author:users (
      id,
      name,
      avatar_url
    ),
    comments (
      id,
      body,
      user:users (name)
    )
  `);
```

#### Filtering

Meridian supports a rich set of filter operators:

```typescript
// Equality
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .eq("status", "published");

// Inequality
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .neq("status", "draft");

// Greater than
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .gt("view_count", 100);

// Range
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .gte("created_at", "2024-01-01")
  .lte("created_at", "2024-12-31");

// Pattern matching
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .like("title", "%tutorial%");

// Case insensitive pattern matching
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .ilike("title", "%Tutorial%");

// In list
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .in("category", ["tech", "science", "design"]);

// Is null
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .is("deleted_at", null);

// Contains (for arrays and JSON)
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .contains("tags", ["javascript", "react"]);
```

#### Filter Operators Reference

| Method | SQL Equivalent | Description |
|--------|---------------|-------------|
| `eq()` | `=` | Equal to |
| `neq()` | `!=` | Not equal to |
| `gt()` | `>` | Greater than |
| `gte()` | `>=` | Greater than or equal |
| `lt()` | `<` | Less than |
| `lte()` | `<=` | Less than or equal |
| `like()` | `LIKE` | Pattern match (case sensitive) |
| `ilike()` | `ILIKE` | Pattern match (case insensitive) |
| `is()` | `IS` | Null check |
| `in()` | `IN` | Value in list |
| `contains()` | `@>` | Contains elements |
| `containedBy()` | `<@` | Contained by |
| `overlap()` | `&&` | Arrays overlap |
| `textSearch()` | `@@` | Full-text search |

#### Insert

```typescript
// Insert a single row
const { data, error } = await meridian.db
  .from("posts")
  .insert({
    title: "Getting Started with Meridian",
    content: "Welcome to the platform...",
    author_id: "user_abc123",
    status: "draft",
  })
  .select();

// Insert multiple rows
const { data, error } = await meridian.db
  .from("tags")
  .insert([
    { name: "javascript", color: "#f7df1e" },
    { name: "typescript", color: "#3178c6" },
    { name: "rust", color: "#dea584" },
    { name: "python", color: "#3776ab" },
  ])
  .select();

// Upsert (insert or update on conflict)
const { data, error } = await meridian.db
  .from("user_settings")
  .upsert({
    user_id: "user_abc123",
    theme: "dark",
    language: "en",
  }, {
    onConflict: "user_id",
  })
  .select();
```

#### Update

```typescript
// Update with filter
const { data, error } = await meridian.db
  .from("posts")
  .update({ status: "published", published_at: new Date().toISOString() })
  .eq("id", "post_123")
  .select();

// Increment a column
const { data, error } = await meridian.db
  .from("posts")
  .update({ view_count: meridian.db.raw("view_count + 1") })
  .eq("id", "post_123");
```

#### Delete

```typescript
// Delete with filter
const { error } = await meridian.db
  .from("posts")
  .delete()
  .eq("id", "post_123");

// Soft delete pattern
const { error } = await meridian.db
  .from("posts")
  .update({ deleted_at: new Date().toISOString() })
  .eq("id", "post_123");
```

#### Ordering and Pagination

```typescript
// Order by column
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .order("created_at", { ascending: false });

// Pagination with range
const { data, count } = await meridian.db
  .from("posts")
  .select("*", { count: "exact" })
  .range(0, 19);

// Cursor-based pagination
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .order("id")
  .gt("id", lastSeenId)
  .limit(20);
```

### Stored Procedures

Call database functions directly:

```typescript
const { data, error } = await meridian.db.rpc("get_monthly_stats", {
  start_date: "2024-01-01",
  end_date: "2024-12-31",
});
```

The corresponding SQL function:

```sql
CREATE OR REPLACE FUNCTION get_monthly_stats(
  start_date DATE,
  end_date DATE
)
RETURNS TABLE (
  month TEXT,
  total_posts BIGINT,
  total_views BIGINT,
  unique_authors BIGINT
)
LANGUAGE sql
STABLE
AS $$
  SELECT
    to_char(date_trunc('month', created_at), 'YYYY-MM') as month,
    count(*) as total_posts,
    sum(view_count) as total_views,
    count(DISTINCT author_id) as unique_authors
  FROM posts
  WHERE created_at BETWEEN start_date AND end_date
    AND status = 'published'
  GROUP BY date_trunc('month', created_at)
  ORDER BY month;
$$;
```

### Row Level Security

Meridian uses Postgres Row Level Security (RLS) to control data access:

```sql
-- Enable RLS on a table
ALTER TABLE posts ENABLE ROW LEVEL SECURITY;

-- Allow users to read published posts
CREATE POLICY "Published posts are visible to everyone"
  ON posts FOR SELECT
  USING (status = 'published');

-- Allow users to manage their own posts
CREATE POLICY "Users can manage own posts"
  ON posts FOR ALL
  USING (auth.uid() = author_id)
  WITH CHECK (auth.uid() = author_id);

-- Admin override
CREATE POLICY "Admins can manage all posts"
  ON posts FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM user_roles
      WHERE user_id = auth.uid()
      AND role = 'admin'
    )
  );
```

## Real-time

Subscribe to database changes, broadcast messages, and track presence across clients.

### Database Changes

Listen to INSERT, UPDATE, and DELETE events on tables:

```typescript
// Subscribe to all changes on a table
const channel = meridian.realtime
  .channel("posts-changes")
  .on(
    "postgres_changes",
    {
      event: "*",
      schema: "public",
      table: "posts",
    },
    (payload) => {
      console.log("Change received:", payload);
      console.log("Event type:", payload.eventType);
      console.log("New record:", payload.new);
      console.log("Old record:", payload.old);
    }
  )
  .subscribe();

// Subscribe to specific events with filters
const channel = meridian.realtime
  .channel("my-posts")
  .on(
    "postgres_changes",
    {
      event: "INSERT",
      schema: "public",
      table: "posts",
      filter: "author_id=eq.user_abc123",
    },
    (payload) => {
      console.log("New post:", payload.new);
    }
  )
  .subscribe();
```

### Broadcast

Send ephemeral messages to all connected clients:

```typescript
// Subscribe to broadcast events
const channel = meridian.realtime
  .channel("game-room")
  .on("broadcast", { event: "cursor-move" }, (payload) => {
    updateCursorPosition(payload.userId, payload.x, payload.y);
  })
  .subscribe();

// Send a broadcast message
channel.send({
  type: "broadcast",
  event: "cursor-move",
  payload: { userId: "user_123", x: 150, y: 320 },
});
```

### Presence

Track which users are online and share state:

```typescript
const channel = meridian.realtime.channel("online-users");

// Track presence
channel.on("presence", { event: "sync" }, () => {
  const state = channel.presenceState();
  console.log("Online users:", Object.keys(state).length);

  for (const [key, presences] of Object.entries(state)) {
    for (const presence of presences) {
      console.log(`${presence.username} is ${presence.status}`);
    }
  }
});

channel.on("presence", { event: "join" }, ({ key, newPresences }) => {
  console.log("User joined:", newPresences);
});

channel.on("presence", { event: "leave" }, ({ key, leftPresences }) => {
  console.log("User left:", leftPresences);
});

// Subscribe and track current user
channel.subscribe(async (status) => {
  if (status === "SUBSCRIBED") {
    await channel.track({
      username: "alice",
      status: "online",
      lastSeen: new Date().toISOString(),
    });
  }
});
```

### Channel Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `broadcast.self` | `boolean` | `false` | Receive own broadcast messages |
| `broadcast.ack` | `boolean` | `false` | Wait for server acknowledgment |
| `presence.key` | `string` | random | Unique key for presence tracking |

### Unsubscribing

```typescript
// Remove a specific channel
await meridian.realtime.removeChannel(channel);

// Remove all channels
await meridian.realtime.removeAllChannels();
```

## Storage

Meridian Storage provides file upload, download, and management with built-in CDN
integration and image transformation capabilities.

### Upload Files

```typescript
// Upload from a File object (browser)
const { data, error } = await meridian.storage
  .from("avatars")
  .upload("users/avatar-123.png", file, {
    contentType: "image/png",
    cacheControl: "3600",
    upsert: true,
  });

// Upload from a buffer (Node.js)
const buffer = fs.readFileSync("./photo.jpg");
const { data, error } = await meridian.storage
  .from("photos")
  .upload("vacation/beach.jpg", buffer, {
    contentType: "image/jpeg",
  });

// Upload with progress tracking
const { data, error } = await meridian.storage
  .from("videos")
  .upload("clip.mp4", file, {
    onUploadProgress: (progress) => {
      const percent = (progress.loaded / progress.total) * 100;
      console.log(`Upload: ${percent.toFixed(1)}%`);
    },
  });
```

### Download Files

```typescript
// Download as blob
const { data, error } = await meridian.storage
  .from("documents")
  .download("reports/annual-2024.pdf");

// Get public URL
const { data } = meridian.storage
  .from("avatars")
  .getPublicUrl("users/avatar-123.png");

console.log(data.publicUrl);

// Get signed URL (temporary access)
const { data, error } = await meridian.storage
  .from("private-docs")
  .createSignedUrl("contracts/nda.pdf", 3600);

// Batch signed URLs
const { data, error } = await meridian.storage
  .from("private-docs")
  .createSignedUrls(["doc1.pdf", "doc2.pdf", "doc3.pdf"], 3600);
```

### Image Transformations

Transform images on-the-fly through URL parameters:

```typescript
const { data } = meridian.storage
  .from("avatars")
  .getPublicUrl("users/avatar-123.png", {
    transform: {
      width: 200,
      height: 200,
      resize: "cover",
      quality: 80,
      format: "webp",
    },
  });
```

Available transform options:

| Option | Type | Description |
|--------|------|-------------|
| `width` | `number` | Target width in pixels |
| `height` | `number` | Target height in pixels |
| `resize` | `"cover" \| "contain" \| "fill"` | Resize mode |
| `quality` | `number` (1-100) | Image quality |
| `format` | `"webp" \| "avif" \| "png" \| "jpeg"` | Output format |

### List Files

```typescript
// List files in a directory
const { data, error } = await meridian.storage
  .from("documents")
  .list("reports/", {
    limit: 100,
    offset: 0,
    sortBy: { column: "created_at", order: "desc" },
    search: "annual",
  });

// Each file object contains:
// {
//   name: "annual-2024.pdf",
//   id: "file_abc123",
//   created_at: "2024-01-15T10:30:00Z",
//   updated_at: "2024-01-15T10:30:00Z",
//   last_accessed_at: "2024-03-01T14:22:00Z",
//   metadata: { size: 1048576, mimetype: "application/pdf" }
// }
```

### Manage Files

```typescript
// Move/rename a file
const { error } = await meridian.storage
  .from("documents")
  .move("old-path/file.pdf", "new-path/renamed.pdf");

// Copy a file
const { error } = await meridian.storage
  .from("documents")
  .copy("source/file.pdf", "backup/file-copy.pdf");

// Delete a file
const { error } = await meridian.storage
  .from("documents")
  .remove(["reports/old-report.pdf"]);

// Delete multiple files
const { error } = await meridian.storage
  .from("temp")
  .remove([
    "upload-1.tmp",
    "upload-2.tmp",
    "upload-3.tmp",
  ]);
```

### Bucket Management

```typescript
// Create a bucket
const { data, error } = await meridian.storage.createBucket("avatars", {
  public: true,
  allowedMimeTypes: ["image/png", "image/jpeg", "image/webp"],
  fileSizeLimit: 5 * 1024 * 1024, // 5MB
});

// List buckets
const { data, error } = await meridian.storage.listBuckets();

// Get bucket details
const { data, error } = await meridian.storage.getBucket("avatars");

// Update bucket
const { data, error } = await meridian.storage.updateBucket("avatars", {
  public: false,
  fileSizeLimit: 10 * 1024 * 1024,
});

// Delete bucket (must be empty)
const { error } = await meridian.storage.deleteBucket("temp-uploads");

// Empty and delete bucket
const { error } = await meridian.storage.emptyBucket("temp-uploads");
await meridian.storage.deleteBucket("temp-uploads");
```

### Storage Policies

Control access to storage objects using SQL policies:

```sql
-- Allow authenticated users to upload to their own folder
CREATE POLICY "Users can upload own files"
  ON storage.objects FOR INSERT
  WITH CHECK (
    auth.uid()::text = (storage.foldername(name))[1]
  );

-- Allow public read access to the avatars bucket
CREATE POLICY "Public avatar access"
  ON storage.objects FOR SELECT
  USING (bucket_id = 'avatars');

-- Allow users to delete their own files
CREATE POLICY "Users can delete own files"
  ON storage.objects FOR DELETE
  USING (
    auth.uid()::text = (storage.foldername(name))[1]
  );
```

## Functions

Meridian Functions let you run server-side code without managing infrastructure. Functions
can be invoked from your client SDK, via HTTP, or on a schedule.

### Invoke a Function

```typescript
// Simple invocation
const { data, error } = await meridian.functions.invoke("hello-world", {
  body: { name: "Alice" },
});

console.log(data.message); // "Hello, Alice!"

// With custom headers
const { data, error } = await meridian.functions.invoke("process-payment", {
  body: {
    amount: 2999,
    currency: "usd",
    customerId: "cust_abc123",
  },
  headers: {
    "X-Idempotency-Key": crypto.randomUUID(),
  },
});
```

### Writing Functions

Functions are written in TypeScript and deployed to the Meridian edge network:

```typescript
// functions/hello-world/index.ts
import { serve } from "@meridian/functions";

serve(async (req) => {
  const { name } = await req.json();

  return new Response(
    JSON.stringify({ message: `Hello, ${name}!` }),
    {
      status: 200,
      headers: { "Content-Type": "application/json" },
    }
  );
});
```

A more complex function with database access:

```typescript
// functions/send-welcome-email/index.ts
import { serve, createClient } from "@meridian/functions";

serve(async (req) => {
  const meridian = createClient(req);
  const { userId } = await req.json();

  // Fetch user from database
  const { data: user, error } = await meridian.db
    .from("users")
    .select("email, name")
    .eq("id", userId)
    .single();

  if (error || !user) {
    return new Response(
      JSON.stringify({ error: "User not found" }),
      { status: 404 }
    );
  }

  // Send email via external service
  const emailResponse = await fetch("https://api.sendgrid.com/v3/mail/send", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${Deno.env.get("SENDGRID_API_KEY")}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      personalizations: [{ to: [{ email: user.email }] }],
      from: { email: "welcome@myapp.com" },
      subject: `Welcome, ${user.name}!`,
      content: [
        {
          type: "text/html",
          value: `<h1>Welcome to MyApp, ${user.name}!</h1>`,
        },
      ],
    }),
  });

  if (!emailResponse.ok) {
    return new Response(
      JSON.stringify({ error: "Failed to send email" }),
      { status: 500 }
    );
  }

  // Log the event
  await meridian.db.from("email_logs").insert({
    user_id: userId,
    type: "welcome",
    status: "sent",
  });

  return new Response(
    JSON.stringify({ success: true }),
    { status: 200 }
  );
});
```

### Scheduled Functions

Run functions on a cron schedule:

```json
{
  "functions": {
    "cleanup-expired-sessions": {
      "schedule": "0 */6 * * *",
      "timeout": 30
    },
    "generate-daily-report": {
      "schedule": "0 8 * * 1-5",
      "timeout": 60
    },
    "sync-external-data": {
      "schedule": "*/15 * * * *",
      "timeout": 120
    }
  }
}
```

```typescript
// functions/cleanup-expired-sessions/index.ts
import { serve, createClient } from "@meridian/functions";

serve(async (req) => {
  const meridian = createClient(req);

  const { count, error } = await meridian.db
    .from("sessions")
    .delete()
    .lt("expires_at", new Date().toISOString());

  console.log(`Cleaned up ${count} expired sessions`);

  return new Response(
    JSON.stringify({ cleaned: count }),
    { status: 200 }
  );
});
```

### Function Configuration

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `timeout` | `number` | `30` | Max execution time in seconds |
| `memory` | `number` | `256` | Memory limit in MB |
| `region` | `string` | `"auto"` | Deployment region |
| `schedule` | `string` | - | Cron expression for scheduled runs |
| `retries` | `number` | `0` | Number of automatic retries |

## Edge Config

Edge Config provides ultra-low-latency key-value storage optimized for reading configuration
data at the edge. Reads typically complete in under 1ms.

### Reading Values

```typescript
import { createEdgeConfig } from "@meridian/edge-config";

const config = createEdgeConfig("ecfg_abc123");

// Get a single value
const featureFlags = await config.get("feature-flags");

// Get multiple values
const [flags, limits, regions] = await config.getAll([
  "feature-flags",
  "rate-limits",
  "allowed-regions",
]);

// Check if a key exists
const exists = await config.has("maintenance-mode");

// Get all keys
const keys = await config.keys();

// Digest (for cache invalidation)
const digest = await config.digest();
```

### Writing Values

Edge Config values are updated through the management API:

```typescript
import { createManagementClient } from "@meridian/sdk";

const mgmt = createManagementClient({ apiKey: "mk_admin_xxx" });

// Set values
await mgmt.edgeConfig.update("ecfg_abc123", {
  items: [
    { operation: "upsert", key: "feature-flags", value: { darkMode: true, beta: false } },
    { operation: "upsert", key: "rate-limits", value: { free: 100, pro: 1000 } },
    { operation: "delete", key: "deprecated-key" },
  ],
});
```

### Common Patterns

#### Feature Flags

```typescript
const flags = await config.get("feature-flags");

if (flags?.darkMode) {
  enableDarkMode();
}

if (flags?.experimentalSearch) {
  renderNewSearchBar();
}
```

#### A/B Testing

```typescript
const experiments = await config.get("experiments");

function getVariant(userId, experimentId) {
  const experiment = experiments[experimentId];
  if (!experiment || !experiment.active) return "control";

  const hash = hashCode(userId + experimentId);
  const bucket = Math.abs(hash) % 100;

  let cumulative = 0;
  for (const [variant, weight] of Object.entries(experiment.variants)) {
    cumulative += weight;
    if (bucket < cumulative) return variant;
  }

  return "control";
}
```

#### Maintenance Mode

```typescript
const maintenance = await config.get("maintenance");

if (maintenance?.active) {
  return new Response(
    renderMaintenancePage(maintenance.message, maintenance.estimatedEnd),
    { status: 503 }
  );
}
```

## CLI Reference

The Meridian CLI provides tools for local development, deployment, and project management.

### Installation

```bash
npm install -g @meridian/cli
```

### Project Commands

```bash
# Initialize a new project
meridian init

# Start local development server
meridian dev

# Build for production
meridian build

# Deploy to production
meridian deploy

# Deploy to preview
meridian deploy --preview

# Link to existing project
meridian link

# View project status
meridian status
```

### Database Commands

```bash
# Start local database
meridian db start

# Stop local database
meridian db stop

# Reset local database
meridian db reset

# Create a new migration
meridian db migration new create_users_table

# Apply migrations
meridian db migration up

# Rollback migrations
meridian db migration down

# Diff local schema vs remote
meridian db diff

# Push local schema to remote
meridian db push

# Pull remote schema to local
meridian db pull

# Seed the database
meridian db seed

# Open database shell
meridian db shell
```

### Function Commands

```bash
# Create a new function
meridian functions new hello-world

# Serve functions locally
meridian functions serve

# Deploy a specific function
meridian functions deploy hello-world

# Deploy all functions
meridian functions deploy --all

# View function logs
meridian functions logs hello-world

# Delete a function
meridian functions delete hello-world
```

### Environment Commands

```bash
# List environment variables
meridian env list

# Set an environment variable
meridian env set STRIPE_KEY=sk_live_xxx

# Unset an environment variable
meridian env unset STRIPE_KEY

# Pull remote env vars to .env.local
meridian env pull
```

### CLI Configuration

The CLI reads from `meridian.toml` in your project root:

```
[project]
id = "proj_abc123"
name = "my-app"

[dev]
port = 54321
db_port = 54322

[functions]
runtime = "deno"
import_map = "./import_map.json"

[deploy]
region = "us-east-1"
```

## Error Handling

All Meridian SDK methods return a consistent error structure. Errors never throw by default -
they are returned as part of the response object.

### Error Structure

```typescript
interface MeridianError {
  message: string;
  code: string;
  details: string | null;
  hint: string | null;
  status: number;
}
```

### Common Error Codes

| Code | Status | Description |
|------|--------|-------------|
| `AUTH_INVALID_CREDENTIALS` | 401 | Invalid email or password |
| `AUTH_SESSION_EXPIRED` | 401 | Session has expired |
| `AUTH_TOKEN_INVALID` | 401 | Invalid or malformed token |
| `AUTH_USER_NOT_FOUND` | 404 | User does not exist |
| `AUTH_EMAIL_TAKEN` | 409 | Email already registered |
| `DB_RELATION_NOT_FOUND` | 404 | Table or view not found |
| `DB_PERMISSION_DENIED` | 403 | RLS policy violation |
| `DB_UNIQUE_VIOLATION` | 409 | Unique constraint violated |
| `DB_FOREIGN_KEY_VIOLATION` | 409 | Foreign key constraint violated |
| `DB_CHECK_VIOLATION` | 400 | Check constraint violated |
| `STORAGE_OBJECT_NOT_FOUND` | 404 | File not found |
| `STORAGE_BUCKET_NOT_FOUND` | 404 | Bucket not found |
| `STORAGE_SIZE_EXCEEDED` | 413 | File too large |
| `STORAGE_INVALID_MIME` | 415 | File type not allowed |
| `FUNCTION_NOT_FOUND` | 404 | Function does not exist |
| `FUNCTION_TIMEOUT` | 504 | Function execution timed out |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |

### Error Handling Patterns

```typescript
// Basic error handling
const { data, error } = await meridian.db
  .from("posts")
  .select("*")
  .eq("id", postId)
  .single();

if (error) {
  switch (error.code) {
    case "DB_RELATION_NOT_FOUND":
      console.error("Table does not exist");
      break;
    case "DB_PERMISSION_DENIED":
      console.error("Access denied - check RLS policies");
      break;
    default:
      console.error("Unexpected error:", error.message);
  }
  return;
}

console.log("Post:", data);
```

```typescript
// Retry pattern for transient errors
async function withRetry(fn, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    const { data, error } = await fn();

    if (!error) return { data, error: null };

    if (error.status >= 500 || error.code === "RATE_LIMIT_EXCEEDED") {
      const delay = Math.min(1000 * Math.pow(2, attempt), 10000);
      console.warn(`Attempt ${attempt} failed, retrying in ${delay}ms...`);
      await new Promise((resolve) => setTimeout(resolve, delay));
      continue;
    }

    return { data: null, error };
  }

  return { data: null, error: { message: "Max retries exceeded", code: "RETRY_EXHAUSTED" } };
}

const { data, error } = await withRetry(() =>
  meridian.db.from("posts").select("*")
);
```

## TypeScript Support

Meridian has first-class TypeScript support. You can generate types from your database schema
for full end-to-end type safety.

### Generate Types

```bash
# Generate types from your project schema
meridian gen types --output ./src/types/database.ts

# Generate types from a specific schema
meridian gen types --schema public --output ./src/types/database.ts
```

### Using Generated Types

```typescript
import { createClient } from "@meridian/sdk";
import type { Database } from "./types/database";

const meridian = createClient<Database>({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
});

// All queries are now fully typed
const { data, error } = await meridian.db
  .from("posts")    // autocomplete for table names
  .select("id, title, status")  // autocomplete for column names
  .eq("status", "published");   // type-checked filter values

// data is typed as Pick<Post, "id" | "title" | "status">[] | null
```

### Custom Type Helpers

```typescript
import type { Database } from "./types/database";

// Extract row types
type Post = Database["public"]["Tables"]["posts"]["Row"];
type InsertPost = Database["public"]["Tables"]["posts"]["Insert"];
type UpdatePost = Database["public"]["Tables"]["posts"]["Update"];

// Use in your application code
async function createPost(post: InsertPost): Promise<Post> {
  const { data, error } = await meridian.db
    .from("posts")
    .insert(post)
    .select()
    .single();

  if (error) throw new Error(error.message);
  return data;
}

async function updatePost(id: string, updates: UpdatePost): Promise<Post> {
  const { data, error } = await meridian.db
    .from("posts")
    .update(updates)
    .eq("id", id)
    .select()
    .single();

  if (error) throw new Error(error.message);
  return data;
}
```

### Type-safe Realtime

```typescript
type RealtimePostPayload = {
  new: Post;
  old: Post;
  eventType: "INSERT" | "UPDATE" | "DELETE";
};

meridian.realtime
  .channel("typed-posts")
  .on<RealtimePostPayload>(
    "postgres_changes",
    { event: "*", schema: "public", table: "posts" },
    (payload) => {
      // payload.new and payload.old are typed as Post
      console.log("Title:", payload.new.title);
    }
  )
  .subscribe();
```

## Migration Guide

### Migrating from v2 to v3

Version 3 introduces several breaking changes to improve consistency and TypeScript support.

#### Client Initialization

```typescript
// v2 (deprecated)
import { createClient } from "@meridian/sdk";
const meridian = createClient("https://proj.meridian.dev", "key");

// v3 (current)
import { createClient } from "@meridian/sdk";
const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
});
```

#### Auth Changes

```typescript
// v2 - auth methods were on the client directly
const { user } = await meridian.signIn({ email, password });
const { user } = await meridian.signUp({ email, password });
await meridian.signOut();

// v3 - auth methods are namespaced
const { user } = await meridian.auth.signIn({ email, password });
const { user } = await meridian.auth.signUp({ email, password });
await meridian.auth.signOut();
```

#### Database Query Changes

```typescript
// v2 - raw filter syntax
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .filter("status", "eq", "published")
  .filter("view_count", "gt", 100);

// v3 - chained method syntax
const { data } = await meridian.db
  .from("posts")
  .select("*")
  .eq("status", "published")
  .gt("view_count", 100);
```

#### Realtime Changes

```typescript
// v2 - callback-based subscriptions
const subscription = meridian.subscribe("posts", (payload) => {
  console.log(payload);
});
subscription.unsubscribe();

// v3 - channel-based subscriptions
const channel = meridian.realtime
  .channel("posts-changes")
  .on("postgres_changes", { event: "*", schema: "public", table: "posts" }, (payload) => {
    console.log(payload);
  })
  .subscribe();

await meridian.realtime.removeChannel(channel);
```

#### Storage Changes

```typescript
// v2
const { publicURL } = meridian.storage.from("avatars").getPublicUrl("avatar.png");

// v3
const { data } = meridian.storage.from("avatars").getPublicUrl("avatar.png");
const publicUrl = data.publicUrl;
```

### Migration Checklist

Use this checklist to track your migration progress:

- [ ] Update `@meridian/sdk` to v3
- [ ] Update client initialization to use object syntax
- [ ] Migrate auth calls to `meridian.auth.*` namespace
- [ ] Replace `.filter()` calls with chained operators
- [ ] Update realtime subscriptions to channel-based API
- [ ] Update storage `getPublicUrl` return type usage
- [ ] Run `meridian gen types` to regenerate type definitions
- [ ] Update custom type helpers for new schema structure
- [ ] Test auth flows (sign in, sign up, sign out, OAuth)
- [ ] Test database queries with new filter syntax
- [ ] Test realtime subscriptions
- [ ] Test file uploads and downloads
- [ ] Run full test suite
- [ ] ~~Update XML response handlers~~ (XML support removed in v3)
- [ ] Deploy to staging environment
- [ ] Verify monitoring and logging
- [ ] Deploy to production

## Advanced Topics

### Connection Pooling

Meridian manages database connections automatically, but you can configure the pool:

```typescript
const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  db: {
    pooling: {
      mode: "transaction",
      maxConnections: 20,
      idleTimeout: 30,
    },
  },
});
```

Pooling modes:

1. **Transaction mode** - Connection is assigned for the duration of a transaction
2. **Session mode** - Connection is assigned for the entire client session
3. **Statement mode** - Connection is assigned for each individual statement

### Batch Operations

For high-throughput scenarios, batch multiple operations:

```typescript
const results = await meridian.db.batch([
  meridian.db.from("users").select("count(*)"),
  meridian.db.from("posts").select("count(*)").eq("status", "published"),
  meridian.db.from("comments").select("count(*)").gt("created_at", "2024-01-01"),
]);

const [usersCount, postsCount, commentsCount] = results;
```

### Custom Fetch Implementation

Override the default fetch for environments with custom networking requirements:

```typescript
import { createClient } from "@meridian/sdk";

const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  global: {
    fetch: (url, options) => {
      // Add custom tracing headers
      const headers = new Headers(options?.headers);
      headers.set("X-Trace-Id", generateTraceId());

      return fetch(url, { ...options, headers });
    },
  },
});
```

### Server-Side Rendering

When using Meridian with SSR frameworks like Next.js or Nuxt:

```typescript
// Next.js App Router example
import { createServerClient } from "@meridian/ssr";
import { cookies } from "next/headers";

export async function createMeridianServerClient() {
  const cookieStore = cookies();

  return createServerClient({
    projectId: process.env.MERIDIAN_PROJECT_ID,
    apiKey: process.env.MERIDIAN_ANON_KEY,
    cookies: {
      get(name) {
        return cookieStore.get(name)?.value;
      },
      set(name, value, options) {
        cookieStore.set(name, value, options);
      },
      remove(name, options) {
        cookieStore.set(name, "", { ...options, maxAge: 0 });
      },
    },
  });
}
```

```typescript
// Usage in a Server Component
import { createMeridianServerClient } from "@/lib/meridian-server";

export default async function PostsPage() {
  const meridian = await createMeridianServerClient();

  const { data: posts } = await meridian.db
    .from("posts")
    .select("id, title, created_at, author:users(name)")
    .eq("status", "published")
    .order("created_at", { ascending: false })
    .limit(20);

  return (
    <div>
      <h1>Published Posts</h1>
      <ul>
        {posts?.map((post) => (
          <li key={post.id}>
            <h2>{post.title}</h2>
            <p>By {post.author.name}</p>
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### Rate Limiting

Meridian applies rate limits per project based on your plan:

| Plan | Requests/sec | Realtime connections | Storage bandwidth |
|------|-------------|---------------------|-------------------|
| Free | 100 | 200 | 1 GB/month |
| Pro | 1,000 | 5,000 | 50 GB/month |
| Team | 5,000 | 20,000 | 250 GB/month |
| Enterprise | Custom | Custom | Custom |

When you hit a rate limit, the SDK returns a `RATE_LIMIT_EXCEEDED` error with a
`Retry-After` header indicating when you can retry.

### Logging and Debugging

Enable verbose logging during development:

```typescript
const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  global: {
    debug: true,
  },
});
```

This outputs detailed logs including:

- All HTTP requests and responses
- WebSocket connection events
- Token refresh operations
- Cache hits and misses

For production monitoring, integrate with your observability platform:

```typescript
import { createClient } from "@meridian/sdk";
import { trace } from "@opentelemetry/api";

const tracer = trace.getTracer("meridian-sdk");

const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  global: {
    fetch: async (url, options) => {
      const span = tracer.startSpan("meridian.request", {
        attributes: {
          "http.url": url.toString(),
          "http.method": options?.method || "GET",
        },
      });

      try {
        const response = await fetch(url, options);
        span.setAttribute("http.status_code", response.status);
        return response;
      } catch (error) {
        span.recordException(error);
        throw error;
      } finally {
        span.end();
      }
    },
  },
});
```

### Caching Strategies

Implement client-side caching for frequently accessed data:

```typescript
const cache = new Map();

async function getCachedData(key, fetcher, ttlMs = 60000) {
  const cached = cache.get(key);
  if (cached && Date.now() - cached.timestamp < ttlMs) {
    return cached.data;
  }

  const { data, error } = await fetcher();
  if (!error && data) {
    cache.set(key, { data, timestamp: Date.now() });
  }

  return data;
}

// Usage
const posts = await getCachedData(
  "published-posts",
  () => meridian.db.from("posts").select("*").eq("status", "published"),
  30000 // 30 second TTL
);
```

For server-side caching with Redis:

```python
import redis
import json
from meridian import create_client

r = redis.Redis(host="localhost", port=6379, db=0)
meridian = create_client(project_id="proj_abc123", api_key="mk_live_xxx")

def get_cached_posts(category):
    cache_key = f"posts:{category}"
    cached = r.get(cache_key)

    if cached:
        return json.loads(cached)

    result = meridian.db.from_("posts") \
        .select("*") \
        .eq("category", category) \
        .eq("status", "published") \
        .execute()

    if result.data:
        r.setex(cache_key, 300, json.dumps(result.data))

    return result.data
```

### Webhooks

Configure webhooks to receive notifications about events in your project:

```typescript
// Webhook handler (Express example)
import express from "express";
import crypto from "crypto";

const app = express();

app.post("/webhooks/meridian", express.raw({ type: "application/json" }), (req, res) => {
  const signature = req.headers["x-meridian-signature"];
  const secret = process.env.WEBHOOK_SECRET;

  // Verify signature
  const hash = crypto
    .createHmac("sha256", secret)
    .update(req.body)
    .digest("hex");

  if (hash !== signature) {
    return res.status(401).json({ error: "Invalid signature" });
  }

  const event = JSON.parse(req.body);

  switch (event.type) {
    case "db.insert":
      console.log("New row inserted:", event.table, event.record);
      break;
    case "db.update":
      console.log("Row updated:", event.table, event.record);
      break;
    case "db.delete":
      console.log("Row deleted:", event.table, event.old_record);
      break;
    case "auth.signup":
      console.log("New user:", event.user.email);
      break;
    case "storage.upload":
      console.log("File uploaded:", event.bucket, event.name);
      break;
  }

  res.json({ received: true });
});
```

### Self-Hosting

Meridian can be self-hosted using Docker Compose:

```bash
# Clone the self-hosting repository
git clone https://github.com/meridian/self-host.git
cd self-host

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Start all services
docker compose up -d

# Check status
docker compose ps

# View logs
docker compose logs -f

# Stop all services
docker compose down
```

The self-hosted stack includes:

- **PostgreSQL** - Primary database
- **PostgREST** - Auto-generated REST API
- **GoTrue** - Authentication server
- **Realtime** - WebSocket server for subscriptions
- **Storage** - S3-compatible file storage
- **Edge Runtime** - Serverless function execution
- **Studio** - Web-based management dashboard

Minimum hardware requirements:

| Component | CPU | RAM | Disk |
|-----------|-----|-----|------|
| All-in-one | 4 cores | 8 GB | 50 GB SSD |
| Database only | 2 cores | 4 GB | 100 GB SSD |
| API + Auth | 2 cores | 2 GB | 10 GB SSD |
| Realtime | 2 cores | 4 GB | 10 GB SSD |
| Storage | 1 core | 1 GB | As needed |

## Frequently Asked Questions

**Q: How does Meridian handle connection failures?**

The SDK includes automatic retry logic with exponential backoff for transient failures.
Network errors, 5xx responses, and rate limits are retried up to 3 times by default. You
can configure this behavior through the client options.

**Q: Can I use Meridian with a existing PostgreSQL database?**

Yes. You can connect Meridian to any PostgreSQL database (version 13 or higher). Use the
`meridian db link` command to connect your existing database, then run `meridian gen types`
to generate TypeScript types from your schema.

**Q: Is there a limit on the number of realtime connections?**

Yes, the limit depends on your plan. Free plans support up to 200 concurrent connections,
Pro plans support 5,000, and Team plans support 20,000. Enterprise plans have custom limits.

**Q: How do I handle file uploads larger than 50MB?**

For large files, use the resumable upload API which supports files up to 5GB:

```typescript
const { data, error } = await meridian.storage
  .from("large-files")
  .uploadResumable("video.mp4", file, {
    chunkSize: 6 * 1024 * 1024, // 6MB chunks
    onProgress: (bytesUploaded, bytesTotal) => {
      const percent = (bytesUploaded / bytesTotal) * 100;
      console.log(`${percent.toFixed(1)}% uploaded`);
    },
    onError: (error) => {
      console.error("Upload failed:", error);
    },
    onSuccess: () => {
      console.log("Upload complete!");
    },
  });
```

**Q: How do I run migrations in production?**

Use the CLI in your CI/CD pipeline:

```bash
# In your deployment script
meridian db migration up --project-id proj_abc123
```

Or use the GitHub Action:

```yaml
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: meridian/setup-cli@v1
        with:
          version: latest
      - run: meridian db migration up
        env:
          MERIDIAN_ACCESS_TOKEN: ${{ secrets.MERIDIAN_ACCESS_TOKEN }}
          MERIDIAN_PROJECT_ID: ${{ secrets.MERIDIAN_PROJECT_ID }}
```

**Q: Can I use Meridian with React Native?**

Yes. Install the SDK along with the async storage adapter:

```bash
npm install @meridian/sdk @meridian/react-native
```

```typescript
import { createClient } from "@meridian/sdk";
import { AsyncStorageAdapter } from "@meridian/react-native";

const meridian = createClient({
  projectId: "proj_abc123",
  apiKey: "mk_live_xxxxxxxx",
  auth: {
    storage: new AsyncStorageAdapter(),
  },
});
```

## Changelog

### v3.2.0 (2025-12-15)

- Added resumable uploads for files up to 5GB
- Added `batch()` method for running multiple queries in parallel
- Improved TypeScript types for realtime payloads
- Fixed race condition in token refresh logic
- Reduced bundle size by 18%

### v3.1.0 (2025-09-01)

- Added Edge Config support
- Added MFA enrollment and verification
- Added phone authentication
- Improved connection pooling defaults
- Fixed WebSocket reconnection in mobile browsers

### v3.0.0 (2025-06-01)

- Breaking: New client initialization syntax
- Breaking: Auth methods moved to `meridian.auth.*` namespace
- Breaking: Database filter methods are now chained
- Breaking: Realtime uses channel-based API
- Added full TypeScript type generation
- Added server-side rendering helpers
- Added custom fetch support
- Removed XML response format support
- Improved error messages and codes

### v2.5.0 (2025-03-01)

- Added webhook support
- Added storage policies
- Improved real-time performance
- Fixed memory leak in long-running subscriptions

### v2.4.0 (2024-12-01)

- Added image transformation API
- Added signed URL batch creation
- Improved query builder performance
- Fixed timezone handling in date filters
