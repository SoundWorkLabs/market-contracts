<div align="center">
  <img style="margin-bottom:15px" src="https://i0.wp.com/soundwork.io/wp-content/uploads/2023/05/2nd-logo_TINY.png?w=1120&ssl=1" height="80px" />
  <h1><strong>Soundwork Marketplace Contracts</strong></h1>
  <p>
    <strong>The Soundwork NFT Marketplace contracts.</strong>
  </p>
  <p>
    <a target="_blank" href="https://discord.gg/Jyw67UfQ"><img alt="Discord Chat" src="https://img.shields.io/badge/chat-discord-blueviolet" /></a>
    <a target="_blank" href="https://github.com/SoundWorkLabs//blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/SoundWorkLabs/market-contracts" /></a>
    <a target="_blank" href="https://www.npmjs.com/package/@jimii/soundwork-sdk"><img alt="SDK" src="https://img.shields.io/npm/v/%40jimii%2Fsoundwork-sdk"/></a>
  </p>
</div>

## Developing

### Environmental Setup

1. Install [Rust](https://rustup.rs/).
2. Install [Solana](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool).
3. Install [Anchor](https://www.anchor-lang.com/docs/installation).

### Install Dependencies

```
yarn
```

### Build the Program

```
anchor build
```

### Testing

```
cargo clippy --all-targets -- -D warnings
anchor test
```
