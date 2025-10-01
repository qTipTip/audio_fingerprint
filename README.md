# Audio Fingerprinting in Rust

An implementation of spectral peak extraction in Rust for audio fingerprinting
of audio.

## Analyze a directory

Here I analyze a directory containing 10 songs in the WAV format. This will
generate fingerprints for each of the 10 songs, and store them all in a binary
file database `audio_fingerprint.db`. Subsequent queries will be done against
this database.

```shell
> cargo run --release analyze-directory -p test_audio/
[2025-10-01T10:26:20Z INFO  audio_fingerprint] Analyzing all .wav files in test_audio/
[2025-10-01T10:26:20Z INFO  audio_fingerprint::fingerprint] Loading fingerprint database
[2025-10-01T10:26:20Z INFO  audio_fingerprint::fingerprint] Database not found, creating new one
[2025-10-01T10:26:21Z INFO  audio_fingerprint::fingerprint] Adding song: 0 with title: test_audio/01_song.wav
[2025-10-01T10:26:21Z INFO  audio_fingerprint::fingerprint] Generating fingerprint
[2025-10-01T10:26:22Z INFO  audio_fingerprint::fingerprint] Done generating fingerprints
[2025-10-01T10:26:22Z INFO  audio_fingerprint::fingerprint] Saving fingerprint database with 1 songs and 126970 fingerprints
...
[2025-10-01T10:26:45Z INFO  audio_fingerprint::fingerprint] Loading fingerprint database
[2025-10-01T10:26:46Z INFO  audio_fingerprint::fingerprint] Adding song: 8 with title: test_audio/09_song.wav
[2025-10-01T10:26:46Z INFO  audio_fingerprint::fingerprint] Generating fingerprint
[2025-10-01T10:26:47Z INFO  audio_fingerprint::fingerprint] Done generating fingerprints
[2025-10-01T10:26:47Z INFO  audio_fingerprint::fingerprint] Saving fingerprint database with 9 songs and 1907980 fingerprints
[2025-10-01T10:26:47Z INFO  audio_fingerprint::fingerprint] Loading fingerprint database
[2025-10-01T10:26:48Z INFO  audio_fingerprint::fingerprint] Adding song: 9 with title: test_audio/10_song.wav
[2025-10-01T10:26:48Z INFO  audio_fingerprint::fingerprint] Generating fingerprint
[2025-10-01T10:26:50Z INFO  audio_fingerprint::fingerprint] Done generating fingerprints
[2025-10-01T10:26:50Z INFO  audio_fingerprint::fingerprint] Saving fingerprint database with 10 songs and 2205747 fingerprints
```

## Recognize a song

For song recognition, I've only tested using a small section of a song analyzed
in the previous step. I used `ffmpeg` to extract a  small section of a song as
follows:

```shell
❯ ffmpeg -ss 38 -t 10s -i test_audio/07_song.wav test_queries/07_song_query.wav
```

The above command will extract the section from 38s to 48s in the song.

I can then attempt to recognize this song in the database:

```shell
❯ cargo run --release recognize -p test_queries/07_song_query.wav
[2025-10-01T10:34:27Z INFO  audio_fingerprint] Attempting to recognize test_queries/07_song_query.wav
[2025-10-01T10:34:27Z INFO  audio_fingerprint::fingerprint] Loading fingerprint database
[2025-10-01T10:34:27Z INFO  audio_fingerprint::fingerprint] Recognizing song
[2025-10-01T10:34:27Z INFO  audio_fingerprint::fingerprint] Generating fingerprint
[2025-10-01T10:34:27Z INFO  audio_fingerprint::fingerprint] Done generating fingerprints
Match found:
Song ID: 6
Title: test_audio/07_song.wav
Confidence: 0.003480129
```

Note the confidence is not really a meaningful metric at this stage, but it
does correspond to the number of votes attributed to the song.
