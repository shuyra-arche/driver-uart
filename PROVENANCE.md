# UART SDK provenance

Source: https://github.com/anreonyr/sqware.git at `f488db5538dc8dd8a530346d76e343708a3c5df0`.

This snapshot extracts the UART byte protocol and Console client only. The UART hardware driver, MMIO handling, scheduling, UnitFile and Source composition remain in Source. Existing wire bytes, bounds, waits, permission and discovery behavior are retained.

Source authors in this history:

- anreonyr <anreonyr@gmail.com>

Source root has no project LICENSE; this extraction invents none and does not rewrite the original history.

Original source snapshots (before import/packaging adaptation):

- `programs/src/driver/uart/client.rs`: SHA256 `031f45a457d1c1b93c428a73fb6cf13c6da8fbef2cbc99577fd70a02bf2fa9d6`.
- `programs/src/driver/uart/core/frame.rs`: SHA256 `835c7f1523f5adcbf0a9461e4d156b0a6c681d2db327d1c4a3606d6d3a3540ff`.

`uart-api` owns the single wire type and names. `uart-client` owns discovery and the two rack endpoint handles. Same-repository paths are allowed; all external component dependencies use public GitHub full revisions. Root native checks and independent DeepSeek review precede public publication.
