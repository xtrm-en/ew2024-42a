# rsty

> A test project for Dioxus frontend developement.

## Requirements

See [REQUIREMENTS.md](./REQUIREMENTS.md)

## Design decisions

Project-wide changes:

- [Nix support](./flake.nix)
  - Added support for Nix as a development shell provider, and (optionally) as a builder
    - This allows the project development/build steps to be fully deterministic and reproducible
    - Docker images can be easily added via the build configuration
  - Note: The Nix setup at 42 is buggy at best, this was mainly setup for development purposes
  
## Encountered issues

- Overall tooling seems to be slow. Is this a known issue at 42?
- Dioxus errors are not very helpful sometimes, which I expected from proc-macros.
