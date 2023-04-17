# Kalshi API with Rust

- [Kalshi API with Rust](#kalshi-api-with-rust)
  - [Overview](#overview)
  - [Kalshi API Documentation](#kalshi-api-documentation)
  - [Setup](#setup)
    - [Environment variables](#environment-variables)
  - [Build](#build)
  - [Contributions](#contributions)

## Overview

Some utilities for working with the Kalshi API in Rust.

## Kalshi API Documentation

https://trading-api.readme.io/reference/

## Setup

### Environment variables

You will need to authenticate with the Kalshi API to use this SDK. Please have a Kalshi username and password available.

- EMAIL - kalshi username / email
- PASS - kalshi password

  ```zsh
  read pass\?"Password: "
  export PASS=$pass
  ```

  or

  ```bash
  read -sp "Password: " pass
  export PASS=$pass
  ```

## Build

The following will produce the lib file and the simple test binary for this repo.

```bash
cargo build
```

## Contributions

If you would like to make contributions, feel free to use the issues tab to report an issue. If you would care to fix the issue yourself and submit a pull request, that is even better.
