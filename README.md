# Safe Remove

## Description
CLI tool similar to linux/ubuntu native `rm` command but backs up the files that are removed. These files can then be restored to their old location.

## Example Usage

### Remove files
```bash
safe-remove remove test.ts ./**/junkfile.txt
```

### Restore files
```bash
safe-remove restore test.ts

safe-remove restore ./nested/dir/junkfile.txt
```

## Why?
I wrote this to get a better feel for coding in Rust and figured this would be a fairly straightforward app.
I'm well aware you could easily use some subset of other commands or tools (mv, cp then rm, etc) to accomplish the same thing.
