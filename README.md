# Fiberplane "Catnip" (tutorial) provider

This repository contains the final code of the provider
built within the "Create a Provider" tutorial.

It relies on the JSON placeholder API to provide features.

## Configuration

```yaml
# It a data_sources.yaml file
- name: tutorial-provider
  description: Tutorial provider
  providerType: catnip
  config:
    endpoint: https://jsonplaceholder.typicode.com
    accept: true
    numRetries: 1
```

## Differences from the tutorial

The differences all lie in the release management:

- there is an additional section in the [cargo configuration](./.cargo/config.toml), that
  adds flags for release builds to shrink the output as much as possible
- there is an additional [one-liner alias](./build_release.sh) that will rebuild a
  release candidate of the catnip provider (a `--release` build that gets another
  pass through `wasm-opt`)
