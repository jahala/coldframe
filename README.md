# coldframe

**The bench where a change's tests run before it goes out.** coldframe looks after the self-hosted GitHub runners on your own Mac and the load they put on it. It also looks after the test tiers each project declares, so every run holds on a busy machine and leaves nothing behind.

It is one binary. There is no server and no daemon: the runners' job hooks run it, LaunchAgents you install run it, and you run it by hand.

**Status: under construction.** The binary builds, checks its own rules and names its version. Its commands land one at a time, in the order of [the map](https://jahala.github.io/coldframe/docs/tend2/coldframe.tend2.html).

```
$ coldframe --version
coldframe 0.1.0
```

## What it will do

| Command | What it does |
|---|---|
| `coldframe status` | Is CI on this Mac healthy? Every runner online, busy or idle, its disk use, and the last prune. Exits non-zero when not. |
| `coldframe prune` | Moves old runner logs, superseded runner versions and update leftovers to the Trash. |
| `coldframe runner adopt` | Points every runner's job hooks at coldframe, restarting each one only while it is idle. |
| `coldframe measure` | The job timeline per repository and for the whole machine: time queued against time running. |
| `coldframe run` | Runs a test tier in its own temporary folder, then reports any file, process or container it left behind. |
| `coldframe survey` | Reads every repository's workflows and runners for known traps, and the month's paid minutes. It changes nothing. |

What coldframe never does is listed in [its guardrails](AGENTS.md#guardrails). Among them: it never restarts a busy runner, never calls GitHub from a job hook, refuses a self-hosted runner on a public repository, and spends nothing.

## Install

macOS, from source. `rust-toolchain.toml` names the Rust version, and `rustup` installs it on the first build.

```bash
git clone https://github.com/jahala/coldframe
cd coldframe
cargo install --locked --path .
```

## Name

A cold frame is the low glass box beside a garden where seedlings harden off before they go outside. coldframe is where a change's tests run, on your own machine, before the change goes out. It is one tool in the plotplot garden.

## Support

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/jahala)

## License

MIT
