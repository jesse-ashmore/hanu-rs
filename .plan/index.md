# Hanu-rs Project Plan

## Overview

A read-only Hacker News client with server-side rendering. Target performance matches news.ycombinator.com while providing extensibility for styling and additional features.

## Tenets

- Text content should be fast, no excuses
- https://news.ycombinator.com is already great
- Re-use, don't re-render
- Make it hard to break
- Make it easy to build more
- Mobile-first design

## Technical Approach

Rendering uses server-side HTML generation per request without hydration. Leptos provides component-driven templating with SSR support. Redis caches API responses with TTL-based expiry. Tailwind CSS handles styling with mobile-first design constraints. The official Hacker News Firebase API serves as the data source.

## Architecture Components

- [API Integration](./api.md) - HN API client and data models
- [Web Server](./web.md) - Leptos components, routing, and rendering
- [Caching Strategy](./cache.md) - Redis integration and TTL policies
- [Deployment](./deploy.md) - Server configuration and hosting

## Current Scope

Initial implementation covers front page story listings, individual story pages with comment threads, user profile pages, and mobile-first responsive design. Access is read-only without authentication or voting. Additional story sources (new, ask, show, jobs) are designed for but deferred. Real-time updates, user authentication, voting functionality, and search are out of scope.

## Key Challenges

### Performance Requirements

Target server response time is 100-200ms matching HN's performance profile. This requires minimizing HTML payload size, optimizing Redis lookup latency, efficient comment tree rendering, and accounting for mobile network variability.

### Data Freshness vs Load

HN API rate limits approximately 10 requests per second per IP. Cache TTLs must balance data staleness against API pressure. Comment threads can exceed 500 items, requiring efficient fetch and render strategies.

### Extensibility Without Complexity

Design must support future story sources without refactoring existing code. Styling variations should be possible without framework lock-in. Feature additions must not degrade performance characteristics.

### Mobile-First Constraints

Touch targets require minimum 44px sizing. Layout prioritizes single-column design. Implementation must handle network variability. Navigation considers thumb-zone ergonomics.

## Development Phases

### Phase 1: Core Infrastructure

Establish project scaffolding with Leptos. Implement Redis connection and cache wrapper. Build HN API client with basic data models. Validate approach with single route proof-of-concept.

### Phase 2: Story Listings

Implement story list component with front page route. Integrate Tailwind CSS for basic styling. Connect cache layer to reduce API calls.

### Phase 3: Story Details

Build comment tree rendering with recursive components. Implement story detail page. Add comment-specific caching strategy.

### Phase 4: Polish

Add user profile pages. Build navigation components. Optimize for mobile devices. Tune performance based on profiling.

## Success Metrics

Server response time under 200ms at p95. HTML payload under 50KB for front page. Zero client-side JavaScript for core functionality. Lighthouse mobile score above 95. Cache hit rate above 90%.
