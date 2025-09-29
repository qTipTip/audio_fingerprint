Running 

```
cargo run --release analyze-directory test_audio/
```

Generates a file with size 33Mb.

## Optimize Fingerprint Struct

Let's try to optimize the fingerprints by using a single 32 bit integer for
each fingerprint. We utilize the following bit layout:

`[10 bit frequency 1][10 bit frequency 2][12 bit time offset]`

We use a 20Hz resolution for the frequency, meaning that each 10 bit chunk can
represent up to 1024 * 20Hz = 20.48KHz.

For the time resolution, we use 5 ms, meaning that each 12 bit chunk can
represent up to 4096 * 5ms = 20480ms = 20.48s. These can be adjusted.

After doing that, we now have a 32 Mb file.

The reason the change is so small is because the
fingerprints are the -keys- to the hasmap, not the values.

