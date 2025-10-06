- [x] Encode a fingerprint as a 32 bit integer.
- [x] Add batch analyze, to analyze multiple files.
- [x] Compute peaks in the 2D time-frequency grid instead of in just the
  frequency domain.
- [x] Load and save the database once per batch operation, instead of on each song.
- [ ] Avoid duplicates in database, by disallowing analyzing the same song twice. (Check for hashes?)
- [ ] Implement support for .mp3 as well.
- [ ] Fetch the sample rate from the WavSpec, instead of hardcoding.
