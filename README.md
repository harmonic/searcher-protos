# Searcher Protos

Minimal protobuf definitions for searchers to submit bundles to the block auction engine.

## Proto Files

- **auth.proto** - Authentication service for obtaining access tokens
- **searcher.proto** - Searcher service for submitting bundles
- **bundle.proto** - Bundle message definition
- **packet.proto** - Packet and metadata definitions
- **shared.proto** - Shared header types

## Authentication Flow

1. Call `AuthService.GenerateAuthChallenge` with `Role.SEARCHER` and your pubkey
2. Sign the challenge: `sign(pubkey || challenge)` with your private key
3. Call `AuthService.GenerateAuthTokens` with the challenge and signature
4. Use the `access_token` in the `Authorization: Bearer <token>` header for API calls
5. When the access token expires, use `AuthService.RefreshAccessToken` with your refresh token

## Submitting Bundles

1. Authenticate (see above)
2. Create a `Bundle` with your transactions as `Packet` messages
3. Call `SearcherService.SendBundle` with the bundle
4. Receive a UUID for tracking

## Example Usage (Rust with tonic)

```rust
// Generate auth challenge
let challenge_req = GenerateAuthChallengeRequest {
    role: Role::Searcher as i32,
    pubkey: keypair.pubkey().to_bytes().to_vec(),
};
let challenge_resp = auth_client.generate_auth_challenge(challenge_req).await?;

// Sign the challenge
let message = [keypair.pubkey().as_ref(), challenge_resp.challenge.as_bytes()].concat();
let signature = keypair.sign_message(&message);

// Get auth tokens
let tokens_req = GenerateAuthTokensRequest {
    challenge: challenge_resp.challenge,
    client_pubkey: keypair.pubkey().to_bytes().to_vec(),
    signed_challenge: signature.as_ref().to_vec(),
};
let tokens_resp = auth_client.generate_auth_tokens(tokens_req).await?;

// Use access token for authenticated requests
let mut request = tonic::Request::new(SendBundleRequest { bundle: Some(bundle) });
request.metadata_mut().insert(
    "authorization",
    format!("Bearer {}", tokens_resp.access_token.unwrap().value).parse().unwrap(),
);
let response = searcher_client.send_bundle(request).await?;
```

## Building

### Rust (with tonic-build)

Add to your `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["proto/auth.proto", "proto/searcher.proto"],
            &["proto"],
        )?;
    Ok(())
}
```

### Other Languages

Use the standard protobuf compiler (`protoc`) with your language's gRPC plugin.
