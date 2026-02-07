# API Integration

## Overview

Integration with the Hacker News Firebase API provides all content data. The API client abstracts HTTP communication and JSON deserialization. A trait-based interface enables testing and future data source alternatives.

## HN API Endpoints

The Firebase API exposes individual item endpoints and collection endpoints.

**Item endpoints:**

- `https://hacker-news.firebaseio.com/v0/item/{id}.json` - Individual item by ID
- `https://hacker-news.firebaseio.com/v0/user/{id}.json` - User profile by username
- `https://hacker-news.firebaseio.com/v0/maxitem.json` - Current largest item ID
- `https://hacker-news.firebaseio.com/v0/updates.json` - Recently changed items and profiles

**Collection endpoints:**

- `topstories.json` - Top/front page stories
- `newstories.json` - Newest stories
- `beststories.json` - Best stories
- `askstories.json` - Ask HN stories
- `showstories.json` - Show HN stories
- `jobstories.json` - Job postings

The API currently has no rate limit. All responses are JSON. No authentication required for read access.

Collection endpoints return up to 500 item IDs. Individual items must be fetched separately. This design requires N+1 requests for displaying N items with details. Comment trees require recursive fetching since each item contains only direct child IDs in the kids array.

## Data Models

### Item

```rust
struct Item {
    id: u32,                        // Unique item identifier
    type: String,                   // "story", "comment", "job", "poll", or "pollopt"
    by: Option<String>,             // Username of author
    time: Option<u64>,              // Unix timestamp
    text: Option<String>,           // HTML content (comments, ask stories, polls)
    dead: Option<bool>,             // True if dead
    deleted: Option<bool>,          // True if deleted
    kids: Option<Vec<u32>>,         // IDs of direct children (comments)

    // Story-specific
    title: Option<String>,          // Story title
    url: Option<String>,            // Story URL (None for ask/poll)
    score: Option<u32>,             // Story score
    descendants: Option<u32>,       // Total comment count

    // Comment-specific
    parent: Option<u32>,            // Parent item ID

    // Poll-specific
    parts: Option<Vec<u32>>,        // Related pollopt IDs
}
```

Deleted items retain only id and deleted fields. Dead items may have minimal data.

### User

```rust
struct User {
    id: String,                     // Username (case-sensitive)
    created: u64,                   // Unix timestamp of account creation
    karma: i32,                     // User karma
    about: Option<String>,          // Bio (HTML)
    submitted: Vec<u32>,            // All submitted item IDs
}
```

Only users with public activity are accessible via the API.

## API Client Interface

```rust
trait HnApi {
    async fn get_story_ids(&self, source: StorySource) -> Result<Vec<u32>>;
    async fn get_item(&self, id: u32) -> Result<Item>;
    async fn get_user(&self, username: &str) -> Result<User>;
    async fn get_max_item(&self) -> Result<u32>;
    async fn get_updates(&self) -> Result<Updates>;
}

struct Updates {
    items: Vec<u32>,
    profiles: Vec<String>,
}
```

The Item struct contains all possible fields with appropriate Option types. Type discrimination happens via the type field. This unified structure matches the API's design where all content is an item.

### StorySource

Story sources are represented as an enum or string constant mapping to API endpoints. Initial implementation supports Top only. The design accommodates New, Best, Ask, Show, Jobs without interface changes.

```rust
enum StorySource {
    Top,
    New,
    Best,
    Ask,
    Show,
    Jobs,
}

impl StorySource {
    fn endpoint(&self) -> &'static str {
        match self {
            Self::Top => "topstories.json",
            Self::New => "newstories.json",
            // ...
        }
    }
}
```

## Error Handling

API errors fall into several categories: network failures (timeouts, DNS), HTTP errors (404, 429 rate limit, 5xx), and deserialization failures (malformed JSON, missing required fields).

The client returns a Result type with custom error enum. Callers decide whether to retry, serve stale cache, or return an error page. Rate limit errors (429) should trigger exponential backoff before retry.

## HTTP Client

Use `reqwest` with connection pooling and reasonable timeouts (5s connect, 10s read). Configure User-Agent header identifying the client. Consider implementing request middleware for logging and metrics.

The client should reuse TCP connections to reduce latency. Connection pool size can start at 10 and be tuned based on profiling.

## Deserialization

Use `serde_json` with derived Deserialize implementations. Mark optional fields with `Option<T>`. Use `#[serde(default)]` for fields that may be missing in older items or deleted content.

Handle HTML entities in text fields. The API returns raw HTML including `<p>` tags and entity-encoded characters. Rendering must preserve or strip these appropriately.

## Testing Strategy

Use `mockall` crate to generate mock implementations of the HnApi trait. This enables unit testing without network calls or external dependencies. Provide test fixtures with representative JSON responses for deserialization validation.

Test error conditions including missing fields, malformed JSON, timeout scenarios, and network failures. Consider recording real API responses for integration tests to validate deserialization against actual data shapes.

## Implementation Notes

Batch requests when fetching multiple items (e.g., top 30 stories). Use tokio tasks to parallelize independent fetches. Avoid sequential iteration that blocks on each request.

The API serves items immutably once created. Stories and comments do not change after initial creation (except score/descendant counts). This immutability simplifies caching strategies.

## Caching

Use Redis for caching API responses. Redis provides fast key-value storage with built-in TTL support and persistence options. Alternative in-memory caches (like moka) could work but Redis enables multi-instance deployments and external cache inspection.

Cache keys follow patterns:

```
stories:{source}     // Story ID lists (e.g., "stories:top")
item:{id}           // Individual items
user:{username}     // User profiles
```

TTL recommendations:

- Story lists: 5 minutes (front page changes frequently)
- Stories: 30 minutes (scores/descendants update slowly)
- Comments: 2 minutes (active threads get new comments)
- Users: 10 minutes (profiles change rarely)

Tune TTLs based on observed traffic patterns. Serve stale cache on API errors for resilience.
