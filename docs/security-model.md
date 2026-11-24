# Security model and production checklist

This project is an anonymous-feedback development foundation. It is not possible to prove that software has no vulnerabilities, so production adoption requires an independent security review, dependency monitoring, deployment controls, and an incident-response process.

## Privacy boundary

The submission contract intentionally accepts only a category and message. It does not accept names, email addresses, student numbers, account IDs, cookies, or browser fingerprint fields. The current development store is in-memory and bounded to 10,000 records; it is lost on restart and should not receive real student data.

Anonymous does not mean untraceable. Network infrastructure, reverse proxies, hosting providers, browser extensions, and operational logs can still create metadata. A production privacy review must define which metadata is retained, who can access it, how long it is retained, and how deletion requests are handled.

## Implemented defenses

- Server-side Unicode message validation and a 16 KiB request-body limit.
- Strict JSON content type enforcement and safe JSON errors that do not echo submitted content.
- Rejection of unknown JSON fields to prevent accidental identifier collection.
- Bounded in-memory retention to reduce memory-exhaustion risk in the development store.
- Fail-closed development admin route with a minimum 32-byte token and constant-time byte comparison.
- Bounded administrative pagination rather than unbounded review responses.
- `Cache-Control: no-store`, clickjacking protection, MIME sniffing protection, restrictive CSP, permissions policy, and no-referrer policy.
- No application logging of feedback bodies, credentials, or personal information.
- Accessible frontend form states, duplicate-submit prevention, and reduced-motion support.

## Required before real deployment

1. Replace the development store with a reviewed database design that defines encryption, retention, deletion, backups, access controls, and row-level authorization.
2. Replace the development header token with institutional identity, phishing-resistant administrator authentication, role-based authorization, session protection, and audit events that do not copy feedback text.
3. Terminate TLS at a trusted edge, configure a strict origin policy, and do not enable permissive CORS. Forward client identity only through a trusted proxy configuration.
4. Add edge rate limiting, abuse detection, request timeouts, connection limits, and a WAF policy. Rate limits must account for anonymous users without relying on spoofable headers.
5. Pin and continuously audit dependencies, generate an SBOM, run secret scanning, and protect the default branch with required checks and reviewed changes.
6. Define privacy notices, consent expectations, data-subject handling, incident response, administrator training, and a tested backup/restore procedure.
7. Run automated integration tests, accessibility checks, dependency scanning, fuzzing of JSON boundaries, and an independent penetration test before collecting real feedback.

## Security reporting

Do not publish credentials, personal data, or working exploit details in an issue. Establish a private security contact and a response SLA before deployment.
