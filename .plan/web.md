# Web Server

## Overview

The web server handles HTTP requests, renders HTML via Leptos components, and coordinates between the API client and cache layer. Server-side rendering generates complete HTML with no client-side hydration for core functionality.

## Framework Selection

Leptos for component-driven rendering, with optional SSR support.

## Server Configuration

```rust
struct AppState {
    api_client: Arc<dyn HnApi>,
    cache: Arc<dyn Cache>,
}
```

Shared state with trait objects for testability. Default Tokio runtime. Target p95 response time under 200ms.

## Routing

```
/              -> StoryList(Top)
/item/:id      -> StoryPage
/user/:id      -> UserPage

Deferred: /new, /best, /ask, /show, /jobs
```

Leptos Router with path parameter extraction. All routes designed in, deferred routes implemented later.

## Component Architecture

```
App
├── Header (Logo, Navigation)
└── Routes
    ├── StoryList → StoryItem (repeating)
    ├── StoryPage → StoryDetail + CommentTree → Comment (recursive)
    └── UserPage → UserProfile
```

All components render server-side with `#[component]` annotation. No hydration.

### Key Components

**StoryList** - Accepts StorySource enum. Fetches first 30 story IDs from cache/API. Renders StoryItem for each.

**StoryItem** - Shows rank, title with link, score, author, time, comment count. Mobile-first readable design.

**StoryPage** - Fetches story and recursively fetches all comments. Renders StoryDetail and CommentTree.

**Comment** - Recursive component with depth parameter for indentation. Cap depth if needed for deep threads.

**UserPage** - Displays karma, about, submission history with pagination or "show more".

## Data Flow and Error Handling

Request → extract params → check cache → (miss) API call → store cache → render component → HTML response. Stale cache serves existing data while refreshing background.

Deleted items show placeholders. Network errors return 503 with retry-after. Malformed IDs return 404. Error pages match HN's minimal aesthetic.

## HTML and Styling

Leptos compiles components to HTML string generation. No virtual DOM. Minimal markup, semantic HTML, proper heading hierarchy.

Tailwind CSS with mobile-first utilities. 44x44px minimum touch targets. Base 16px text. Responsive variants (sm:, md:, lg:) for progressive enhancement. Consider thumb zones for navigation.

## Performance

Fast component rendering. Pre-compute derived data. Profile with criterion. HTML payload under 50KB for lists. Gzip/brotli compression.

## Assets and Development

Tailwind generates CSS bundle. No JS required for core features.

Use cargo-leptos for development with hot reload.
