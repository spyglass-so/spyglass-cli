# Spyglass CLI Reference

Full command reference for `spyglass`.

## Commands

- [`spyglass account`](#spyglass-account)
- [`spyglass agent`](#spyglass-agent)
- [`spyglass brands`](#spyglass-brands)
- [`spyglass comments`](#spyglass-comments)
- [`spyglass creators`](#spyglass-creators)
- [`spyglass media`](#spyglass-media)
- [`spyglass taxonomies`](#spyglass-taxonomies)
- [`spyglass team`](#spyglass-team)

---

### `spyglass account`

#### `spyglass account campaigns`

Return an account’s campaigns or offers, newest start date first.
`datePreset`, or `startDate` and `endDate` as `YYYY-MM-DD`, filter
by campaign start date. Omit both for all time.

Limited to 120 requests per minute per user.
Costs $0.0010 per campaign returned, charged against your plan's monthly credits. The count field is what you were charged for.

`GET /account/{id}/campaigns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |
| `--type` | `CAMPAIGN | OFFER` | Yes |  |
| `--sort-by` | `MOST_RECENT | HIGHEST_VELOCITY` | No |  |
| `--limit` | `integer` | No |  |
| `--date-preset` | `today | yesterday | last-3-days | last-7-days | this-week | last-week | last-14-days | last-30-days | this-month | last-month | last-3-months | last-6-months | year-to-date | last-year` | No |  |
| `--start-date` | `string (date)` | No |  |
| `--end-date` | `string (date)` | No |  |

#### `spyglass account collabs`

Return the creators an account has collaborated with, ordered from
most media to least. Works for both brands and creators.
mediaCount is how many of that account’s media items tag the
creator. startedAt is the earliest of those media start times, and
lastPostedAt is the latest. `datePreset`, or `startDate` and
`endDate` as `YYYY-MM-DD`, filter by startedAt. Omit both for all time.

Limited to 120 requests per minute per user.
Costs $0.0010 per collab returned, charged against your plan's monthly credits. The count field is what you were charged for.

`GET /account/{id}/collabs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |
| `--date-preset` | `today | yesterday | last-3-days | last-7-days | this-week | last-week | last-14-days | last-30-days | this-month | last-month | last-3-months | last-6-months | year-to-date | last-year` | No |  |
| `--start-date` | `string (date)` | No |  |
| `--end-date` | `string (date)` | No |  |

#### `spyglass account get`

Fetch a single brand or creator. `id` is any one of:

- an account id (a UUID, or the numeric id from POST /brands or POST /creators)
- a website URL or bare domain, such as `https://www.nike.com` or `nike.com`
- an Instagram handle, such as `nike` or `@nike`
- a TikTok handle, such as `nike` or `@nike`

A website matches the account domain. A handle matches `igUsername` or
`tiktokUsername`. When both match, the `igUsername` match is returned.
Encode `/` and `?` in a URL. The `type` field tells you which
one you got: `BRAND` or `CREATOR`, and the creator-only fields
(`gender`, `ageRange`, ...) are present only on `CREATOR`.

Limited to 120 requests per minute per user.
Costs $0.0010 per account returned, charged against your plan's monthly credits.
An id that matches nothing returns 404 and is free.

`GET /account/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |

#### `spyglass account index`

Look up a brand by website URL, or start indexing it if we do not
have it yet. Pass a single `websiteUrl`. Instagram and TikTok
profile URLs are accepted and resolved as creators.

An existing brand returns
`status: "already_exists"`. A new brand is created and sync is
queued, returning `status: "indexing"` and the new `brandId`.

Limited to 50 requests per minute per user.
This call is free and does not consume credits.

`POST /account/index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

#### `spyglass account index-status`

Poll whether a brand finished indexing after POST /account/index.

Limited to 300 requests per minute per user.
This call is free and does not consume credits.

`GET /account/{id}/index-status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |

#### `spyglass account insights`

Return creative insights for an account. `MEDIA_MIX` counts creatives by
media type, `CATEGORY` by creative category, and `FUNNEL` by funnel
stage. `HOOK`, `TARGET_PERSONA`, and `AD_ANGLE` return ranked totals
(`id`, `label`, `tags`, `total`), highest first.

If a hook, angle, or persona clustering run is missing, older than 30
days, or has no clusters, clustering starts and the response includes
`status: "analyzing"` with empty totals. Retry for the finished clusters.
Analyzing responses are not charged.

`datePreset`, or `startDate` and `endDate` as `YYYY-MM-DD`, filter
by creative start date. Omit both for the last 30 days.

Limited to 120 requests per minute per user.
Costs $0.0010 per cluster returned, charged against your plan's monthly credits. The count field is what you were charged for.

`GET /account/{id}/insights`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |
| `--type` | `HOOK | TARGET_PERSONA | AD_ANGLE | MEDIA_MIX | CATEGORY | FUNNEL` | Yes |  |
| `--date-preset` | `today | yesterday | last-3-days | last-7-days | this-week | last-week | last-14-days | last-30-days | this-month | last-month | last-3-months | last-6-months | year-to-date | last-year` | No |  |
| `--start-date` | `string (date)` | No |  |
| `--end-date` | `string (date)` | No |  |
| `--limit` | `integer` | No |  |

#### `spyglass account landing-pages`

Return the landing-page URLs an account sends ad traffic to, ranked by
how often they appear across its ads.

Limited to 120 requests per minute per user.
Costs $0.0010 per landing page returned, charged against your plan's monthly credits. The count field is what you were charged for.

`GET /account/{id}/landing-pages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | Any one of: an account id, a website, an Instagram handle, or a TikTok handle. An id is a UUID or the numeric id from POST /brands or POST /creators. A website may be a full URL or a bare domain. A handle may start with @. Encode `/` and `?` when the value is a URL. |
| `--show-inactive` | `true | false` | No |  |
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |

#### `spyglass account posts`

Return one account's posts, fetched live. The account does not need to be indexed.
A call usually takes 10–30 seconds and starts indexing the rest of the brand's posts.
Use `POST /media` first when you already have an account id. See the Finding media guide.

Limited to 120 requests per minute per user.
Costs $0.0010 per creative returned, charged against your plan's monthly credits. The count field is what you were charged for.
The call bills a minimum of one row even when nothing matches. In that
case `count` is 0 but you are charged for one row.

Send exactly one identifier. The fields you can send depend on which
one you pick.

**`igUsername`**

| Field | Details |
| --- | --- |
| `igUsername` | Instagram handle. A leading `@` is ignored. |
| `tagged` | Optional. `true` returns posts the account is tagged in. Omit it for the account's own posts. Cannot be combined with `postType=reels`. |
| `postType` | Optional, Instagram only. `posts` (default) or `reels`. Cannot be combined with `tagged`. |
| `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
| `limit` | Optional. Default 20, max 120. |
| `sortBy` | Ignored. Posts are always most recent. |

**`tiktokUsername`**

| Field | Details |
| --- | --- |
| `tiktokUsername` | TikTok handle. A leading `@` is ignored. |
| `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
| `sortBy` | Optional. `most_recent` (default) or `popular`. |
| `limit` | Optional. Default 20, max 120. |

**`facebookAdLibraryId`**

| Field | Details |
| --- | --- |
| `facebookAdLibraryId` | Facebook Ad Library page id. |
| `includeTranscript` | Optional. `true` transcribes each video and returns `transcript`. Omit it or send `false` to skip transcription. The call takes longer. |
| `sortBy` | Optional. `most_recent` (default) or `most_impressions`. |
| `limit` | Optional. Default 20, max 120. |

`GET /account/posts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--ig-username` | `string` | No |  |
| `--tiktok-username` | `string` | No |  |
| `--facebook-ad-library-id` | `string` | No |  |
| `--tagged` | `true | false` | No |  |
| `--post-type` | `posts | reels` | No |  |
| `--include-transcript` | `true | false` | No |  |
| `--sort-by` | `popular | most_impressions | most_recent` | No |  |
| `--limit` | `integer` | No |  |

---

### `spyglass agent`

#### `spyglass agent create`

Start a Spyglass agent run. The run id is the chat thread id.
Pass `outputSchema` to get that JSON on `structured` when the run completes.
The root `type` must be `"object"`. Each property can be a string, number, integer, boolean, array, or object.
`model` is `default` or `pro`.
Poll GET /agent/runs/{id} until `status` is `completed` or `failed`.

Limited to 20 requests per minute per user.
The create call is free. The run is billed from model usage when it finishes.

`POST /agent/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

#### `spyglass agent get`

Poll a run until `status` is `completed` or `failed`.
Completed runs include `text` and, when you passed a schema, `structured`.
Failed runs include `error` with a `code` and `message`.

Limited to 120 requests per minute per user.

`GET /agent/runs/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes |  |

#### `spyglass agent list`

Lists runs for this team, newest first.
Page with `offset`. The next page is `offset` plus the `limit` you sent.
`count` is the number of runs in this response. Stop when `count` is less than `limit`.

Limited to 120 requests per minute per user.
This call is free and does not consume credits.

`GET /agent/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No |  |
| `--offset` | `integer` | No |  |

---

### `spyglass brands`

#### `spyglass brands search`

Find brands.
Filter by taxonomy, location, or followers.

Leave the search fields empty to list by `sortBy` (default `MOST_FOLLOWERS`).
Or pass one of:

- `query` — semantic search. Ordered by relevance, so `sortBy` is ignored.
  Bills at least one row even when nothing matches.
- `names` — name contains this text.
- `ids` — UUID, handle, domain, or Ad Library id. Misses are in
  `unmatchedIds`. Ignores `limit` and `offset`.
- `igUsername` or `tiktokUsername` — handle contains this text. A leading
  `@` is ignored.

When more than one is set:

`ids` > `igUsername` / `tiktokUsername` > `names` > `query`

`names` + `query` uses both.

`limit` is 20 by default, 100 max, and caps the charge except for `ids`.

Limited to 120 requests per minute per user.
Costs $0.0010 per brand returned, charged against your plan's monthly credits. The count field is what you were charged for.

`POST /brands`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

---

### `spyglass comments`

#### `spyglass comments get`

Return public comments for specific Instagram or TikTok posts.
Pass post URLs or the platform id. At least one of `instagramIds` or
`tiktokIds` is required (max 20 each). `limit` is comment pages
per post (default 1, max 20).

Instagram accepts a shortcode or a post URL:

- `C2xY9abcD1e`
- `https://www.instagram.com/p/C2xY9abcD1e/`
- `https://www.instagram.com/reel/C2xY9abcD1e/`

TikTok accepts a video id or a post URL:

- `7234567890123456789`
- `https://www.tiktok.com/@nike/video/7234567890123456789`

Recent posts may be scraped live; older posts with stored comments
are served from cache.

Limited to 20 requests per minute per user.
Costs $0.0050 per successful page of comments returned, charged against your plan's monthly credits.
Each comment page fetched live (`limit`, default 1) is one billed
unit. Cached posts do not scrape and are not billed.

`POST /comments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

---

### `spyglass creators`

#### `spyglass creators search`

Find creators.
Filter by category, audience, location, or followers.

Leave the search fields empty to list by `sortBy` (default `MOST_FOLLOWERS`).
Or pass one of:

- `query` — semantic search. Ordered by relevance, so `sortBy` is ignored.
  Bills at least one row even when nothing matches.
- `names` — name contains this text.
- `ids` — UUID, handle, domain, or Ad Library id. Misses are in
  `unmatchedIds`. Ignores `limit` and `offset`.
- `igUsername` or `tiktokUsername` — handle contains this text. A leading
  `@` is ignored.

When more than one is set:

`ids` > `igUsername` / `tiktokUsername` > `names` > `query`

`names` + `query` uses both.

`limit` is 20 by default, 100 max, and caps the charge except for `ids`.

Limited to 120 requests per minute per user.
Costs $0.0010 per creator returned, charged against your plan's monthly credits. The count field is what you were charged for.

`POST /creators`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

---

### `spyglass media`

#### `spyglass media search`

Return media posts. A call usually takes under 10 seconds.

**1. Pick what to search**

| You want | Send |
| --- | --- |
| All media | Send neither `accountId` nor `query` to list media across the index, ordered by `sortBy` |
| One account's media | `accountId` |
| Media matching a description | `query`, which matches posts by what they show or say |
| One account's media matching a description | `accountId` and `query` |
| A broader category | `taxonomy`, a path from GET /taxonomies |

**2. Pick a sort order**

`sortBy` defaults to `MOST_RECENT`. Some orders only work with certain channels:

| `sortBy` | Allowed `channel` |
| --- | --- |
| `MOST_RECENT`, `OLDEST` | Any |
| `MOST_IMPRESSIONS`, `LONGEST_RUN` | Exactly `META_AD` |
| `MOST_VIEWS`, `MOST_LIKES`, `MOST_COMMENTS`, `HIGHEST_ENGAGEMENT`, `HIGHEST_OUTLIER` | Anything except `META_AD` |

**3. Optionally add a transcript or caption**

Send `includeTranscript: true` to get each video transcript.
Send `includeCaption: true` to get each post caption.
Both default to false and do not change the price.

**Cost and limits**

Costs $0.0010 per creative returned, charged against your plan's monthly credits. The count field is what you were charged for.
You are always charged for at least one row. If nothing matches, `count` is 0 but you pay for one row.
Limited to 120 requests per minute per user.

`POST /media`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON|-` | Yes | Request body as JSON (or `-` to read from stdin) |

---

### `spyglass taxonomies`

#### `spyglass taxonomies search`

Find taxonomy paths you can pass to POST /media as `taxonomy`.
`query` is a product or category phrase. Each path is a string
such as `Apparel > Footwear > Sneakers`.

Limited to 120 requests per minute per user.
This call is free and does not consume credits.

`GET /taxonomies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | Yes |  |
| `--limit` | `integer` | No |  |

---

### `spyglass team`

#### `spyglass team credit-usage`

Credits left on the API key's team, the credits included in the
plan, and the current billing period. Granted credits count toward
`remainingCredits` and not `planCredits`.

Limited to 60 requests per minute per user.
This call is free and does not consume credits. It still responds when
the team is out of credits.

`GET /team/credit-usage`

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--no-extract` | Print the full response body instead of only the part the command returns by default |
| `--no-retry` | Disable automatic retries, including retries on network errors |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

Paginated commands also accept:

| Flag | Description |
|------|-------------|
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `--page-delay <MS>` | Delay between page fetches in milliseconds (default: `100`) |
| `--no-pager` | Disable the pager even on interactive terminals |

Streaming commands also accept:

| Flag | Description |
|------|-------------|
| `--no-stream` | Buffer the streaming response and print it as a single value once complete |
