# Issues

## Memory Layout

- **Arena-backed field types for zero-copy decoding:** Currently packets use owned `String` and `Vec<u8>`,
  which means each field is its own allocation. For decoding-heavy workloads this creates allocation pressure
  and poor cache locality across a packet tree. Two viable directions: (a) adopt `bytes::Bytes` for byte
  fields and `Arc<str>` (or a `Bytes`-backed UTF-8 newtype) for string fields, with the decoder reading the
  entire input into one `Bytes` and slicing out fields — zero-copy, single allocation, integrates with
  `tokio`/`tonic` ecosystems; or (b) build a custom `Arena` type plus `ArenaStr`/`ArenaBytes` owned-handle
  types that share an `Arc<Arena>` backing store. Option (a) is the path of least resistance and likely
  sufficient. Option (b) gives smaller per-field overhead and full control but requires ~200-300 lines of
  arena/handle/decoder-context infrastructure. Either option is compatible with the existing `Send + Sync`
  bound on `Packet` and would let `+ 'static` be added back. Decision deferred — needs profiling to know
  whether the optimization is worth the complexity.

## Documentation

- **Missing crate-level doc comment in `lib.rs`:** As a published crate pitched as a protobuf alternative, the
  crate root should have a `//!` block describing the crate's purpose and ideally a short example. This is what
  docs.rs visitors see first.

## Decoding

- **Decoder error precision is poor across the runtime crate.** Multiple decode-side failure paths funnel
  through `enc::Error::InvalidEncodedData { reason: None }`, which loses both the *kind* of failure and
  the *failing value*. Known sites:
  - `transfer.rs:60` — `transfer_varint` rejecting an overlong varint (also has a `// todo -- better error
    here` marker).
  - `impl_enum_decode.rs:19` — enum decode rejecting an out-of-range tag number.
  - `field_header.rs:121` — `FieldHeader` decode rejecting an out-of-range tag number.
  - Probably more across other decoders not yet audited.

  Wants a single coherent error-type redesign rather than piecemeal fixes. Likely shape: either richer
  variants on `enc::Error` (e.g., `InvalidTagNumber(u32)`, `OverlongVarInt`), a runtime-crate-local error
  wrapper that adds proto-packet-specific variants, or filling in the existing `reason: Option<...>` field
  with a structured value. Should be tackled as one piece of work touching all sites at once.

- **Unbounded allocation in `WireType::decode_length_prefixed_bytes`:** The varint length prefix is
  attacker-controlled, and `vec![0; prefix]` allocates exactly that many bytes upfront — before any payload
  is read. A malicious encoder can declare a multi-GB length and OOM the decoder without sending the actual
  payload bytes. `list_header.rs::MAX_CAPACITY_HINT` defends against this for lists; the bytes case has no
  equivalent cap. Likely fix: replace `vec![0; prefix]` with `Read::take(cap).read_to_end(&mut vec)` plus a
  hard cap (e.g., 64 MB), so memory grows only as bytes are actually delivered. Decision needed on the cap
  value and whether it should be per-`Decoder` or a constant.

- **Decoding `Vec<bool>` fails with `InvalidWireType`:** Decoding a struct containing a `Vec<bool>` field fails with
  `InvalidWireType { semantic: "Vec<bool>", wire: List }`. Encoding works correctly but the round-trip decode rejects
  the wire type. The encoder uses `LengthPrefixed` for `Vec<bool>` but the decoder expects `List`.

## Encoding

- **Redundant `encoded_len()` calls:** When encoding `Vec<P>` of length-prefixed packets, each element's
  `encoded_len()` is called twice — once during `elements_len()` to size the `ListHeader`, then again inside
  `Encoder<P>::encode_to_slice_unchecked` / `encode_to_write` to compute the per-element length prefix. For
  nested packets the cost compounds with depth. Likely fix: cache `encoded_len()` on the packet itself so
  the second call is free.

- **`Packet::wire()` should be an associated constant:** Currently `Packet::wire()` is a function returning
  `WireType`, so `if P::wire() == LengthPrefixed { ... }` in `Encoder<P>`'s impls is a runtime equality check
  per call. If it were `const WIRE: WireType` instead, the comparison would be a per-monomorphization compile
  time constant and LLVM would dead-code-eliminate the unused branch entirely. Affects every encode and decode
  path for `Packet` types.

## Services

- **Streaming:** The request and response are buffered there isn't really streaming support.
- **Async:** The scope is async with actix-web but the service is not.
- **Comments:** The comments on service calls don't propagate to generated code.
- **Errors:** Error handling is rudimentary at best, it could be better than text.
- **Routing:** All calls use POST and are hard coded paths. Some calls might want GET.
- **Middleware:** There are no middleware hooks on services or service calls.
- **Clients:** There is no client generation, only server generation.
- **Actix Only:** The only supported web-framework is actix-web.
