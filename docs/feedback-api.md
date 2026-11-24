# Feedback API contract (development foundation)

This document defines the first API contract before a persistent data store or
identity provider is introduced. It is deliberately small so the validation
and authorization boundaries are clear and testable.

## Submit feedback

`POST /api/feedback`

```json
{
  "category": "teaching",
  "message": "The weekly examples made the topic much easier to understand."
}
```

- `category` must be one of `teaching`, `course-content`, `facilities`, or
  `other`.
- `message` is required after trimming and may contain 1–2,000 Unicode
  characters.
- No other JSON fields are accepted. This keeps direct identifiers out of the
  anonymous contract unless the anonymity policy is deliberately changed.
- The endpoint does not accept a name, email address, account identifier, or
  other direct identifier. Do not add those fields without an approved
  anonymity policy.
- A successful submission receives `201 Created` and an opaque feedback ID.

The request must use `Content-Type: application/json`. Invalid JSON and
unexpected fields return `400 Bad Request`; missing or invalid message values
return `422 Unprocessable Entity`; other media types return `415 Unsupported
Media Type`.

## Administrative review

`GET /api/admin/feedback`

The development server protects this route with an `x-admin-token` header.
It is disabled unless `ADMIN_REVIEW_TOKEN` is configured with at least 32
bytes. Token comparison does not return early on the first mismatched byte.
This is a temporary development guard, **not** a replacement for institutional
authentication, role-based authorization, or row-level database policies.

Review results are bounded. Use `?limit=50&offset=0`; the default limit is 50
and the maximum limit is 100. Results are ordered by insertion order in this
development store.

API responses set `Cache-Control: no-store`, `X-Content-Type-Options: nosniff`,
`X-Frame-Options: DENY`, a restrictive content security policy, and a
no-referrer policy. The server also rejects request bodies over 16 KiB.

## Error responses

Expected client errors return JSON in this form:

```json
{
  "error": "validation_failed",
  "message": "message must not be empty"
}
```

The `error` value is one of `validation_failed`, `invalid_json`, or
`unsupported_media_type` for the documented submission failures.

The API never includes the submitted feedback text in an error response or
application log.
