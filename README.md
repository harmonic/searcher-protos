# Searcher Protos

Protobuf definitions for searchers to submit bundles.

> **Important:** The `Role.SEARCHER` enum value is `3` in this repo, unlike Jito protos where `SEARCHER = 1`. Make sure to use `Role.SEARCHER` (not a hardcoded value) to avoid compatibility issues.

## Proto Files

- `auth.proto` - Authentication service for obtaining access tokens
- `searcher.proto` - Searcher service for submitting bundles
- `bundle.proto` - Bundle message definition
- `packet.proto` - Packet and metadata definitions
- `shared.proto` - Shared header types

## Usage

Use `protoc` with your language's gRPC plugin to generate client code:

```bash
protoc --proto_path=proto \
  --go_out=. --go-grpc_out=. \
  proto/auth.proto proto/searcher.proto
```

For Rust with tonic, add to your `build.rs`:

```rust
tonic_build::configure()
    .build_server(false)
    .compile(&["proto/auth.proto", "proto/searcher.proto"], &["proto"])?;
```

## Authentication

1. Call `AuthService.GenerateAuthChallenge` with `Role.SEARCHER` (3) and your pubkey
2. Sign the challenge: `sign(pubkey || challenge)` with your private key
3. Call `AuthService.GenerateAuthTokens` with the challenge and signature
4. Use the `access_token` in the `Authorization: Bearer <token>` header
5. Refresh with `AuthService.RefreshAccessToken` when the token expires

## Submitting Bundles

1. Authenticate (see above)
2. Create a `Bundle` with transactions as `Packet` messages
3. Call `SearcherService.SendBundle`
4. Receive a UUID for tracking
