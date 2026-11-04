# UART SDK provenance

## Initial SDK extraction (historical)

Source: https://github.com/anreonyr/sqware.git at `f488db5538dc8dd8a530346d76e343708a3c5df0`.

This snapshot extracts the UART byte protocol and Console client only. The UART hardware driver, MMIO handling, scheduling, UnitFile and Source composition remain in Source. Existing wire bytes, bounds, waits, permission and discovery behavior are retained.

Source authors in this history:

- anreonyr <anreonyr@gmail.com>

Source root has no project LICENSE; this extraction invents none and does not rewrite the original history.

Original source snapshots (before import/packaging adaptation):

- `programs/src/driver/uart/client.rs`: SHA256 `031f45a457d1c1b93c428a73fb6cf13c6da8fbef2cbc99577fd70a02bf2fa9d6`.
- `programs/src/driver/uart/core/frame.rs`: SHA256 `835c7f1523f5adcbf0a9461e4d156b0a6c681d2db327d1c4a3606d6d3a3540ff`.

`uart-api` owns the single wire type and names. `uart-client` owns discovery and the two rack endpoint handles. Same-repository paths are allowed; all external component dependencies use public GitHub full revisions. Root native checks and independent DeepSeek review precede public publication.

## UART runtime extraction

Source: https://github.com/anreonyr/sqware.git at `94e114614bec53424ac55e640fc80cf19a3e7107`.

Repository baseline for this extraction: `b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`.

This addition extracts the UART runtime provider and its assembly declaration.
`uart-server` owns the unchanged runtime loop through public `serve()` and a
thin `#[mold::entry]` binary; shared Context/Hub/Device/Fail come from public
`driver-shared` at
`e75b1e232f828666956fb7e02c61a7d84f13eb66`, and the wire type/names come from
the same-repository `uart-api`. `uart-declaration` owns the single `UnitFile`
and the server build descriptor. No new driver support crate is introduced.

Source snapshots (before provider/packaging adaptation):

- `programs/src/driver/uart/main.rs`: SHA256 `8bc44863011ee267cf0ccaba11577b73ca6697d88ec69c66b70d76c383b0b371`.
- `programs/src/driver/uart/adapt/desk.rs`: SHA256 `b0f755fd6e33057a9e0b2c8da5440a4241760f2a5a72f66dab8ca2013481cb2c`.
- `programs/src/driver/uart/dev/uart.rs`: SHA256 `f3c845a07b70b563e559690dc0a02721faebb2f6b3be2a035438e4a42f9778e9`.
- `crates/composition/src/units/uart.rs`: SHA256 `99fecaf0cfbceb950d0750765b92548a99d19018f530610c9ea534208613870a`.

No dedicated historical UART server host suite was found in Source. The rack
probe programs are integration harnesses and remain in Source; they were not
copied here. `tests/declaration` is a new host contract test for the actual
declaration crate, not a copied Source suite.
