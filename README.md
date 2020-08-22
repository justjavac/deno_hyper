# deno_hyper

[![tag](https://img.shields.io/github/release/justjavac/deno_hyper)](https://github.com/justjavac/deno_hyper/releases)
[![Build Status](https://github.com/justjavac/deno_hyper/workflows/ci/badge.svg?branch=master)](https://github.com/justjavac/deno_hyper/actions)
[![license](https://img.shields.io/github/license/justjavac/deno_hyper)](https://github.com/justjavac/deno_hyper/blob/master/LICENSE)

Fast and safe HTTP, [Hyper](https://hyper.rs) binding for Deno.

## Usage

```ts
import { run } from "https://deno.land/x/hyper/mod.ts";

run();
```

run example:

```bash
deno run -A --unstable https://deno.land/x/hyper/example.ts
```

## Flags

- `--unstable`
- `--allow-plugin`

### License

[deno_hyper](https://github.com/justjavac/deno_hyper) is released under the MIT License. See the bundled [LICENSE](./LICENSE) file for details.
