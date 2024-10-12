# Command-Line Help for `caesura`

This document contains the help content for the `caesura` command-line program.

**Command Overview:**

* [`caesura`↴](#caesura)
* [`caesura config`↴](#caesura-config)
* [`caesura batch`↴](#caesura-batch)
* [`caesura spectrogram`↴](#caesura-spectrogram)
* [`caesura transcode`↴](#caesura-transcode)
* [`caesura upload`↴](#caesura-upload)
* [`caesura verify`↴](#caesura-verify)

## `caesura`

An all-in-one command line tool to **transcode FLAC** audio files and **upload to gazelle** based indexers/trackers. 

**Usage:** `caesura [COMMAND]`

###### **Subcommands:**

* `config` — Generate a config.json file in the current working directory
* `batch` — Verify, transcode, and upload from multiple FLAC sources in one command
* `spectrogram` — Generate spectrograms for each track of a FLAC source
* `transcode` — Transcode each track of a FLAC source to the target formats
* `upload` — Upload transcodes of a FLAC source
* `verify` — Verify a FLAC source is suitable for transcoding



## `caesura config`

Generate a config.json file in the current working directory

**Usage:** `caesura config`



## `caesura batch`

Verify, transcode, and upload from multiple FLAC sources in one command

**Usage:** `caesura batch [OPTIONS] [SOURCE]`

###### **Arguments:**

* `<SOURCE>` — Source as: torrent id, path to torrent file, or indexer url.

   Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

###### **Options:**

* `--api-key <API_KEY>` — API key
* `--indexer <INDEXER>` — ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red
* `--indexer-url <INDEXER_URL>` — URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer
* `--announce-url <ANNOUNCE_URL>` — Announce URL including passkey Examples: https://flacsfor.me/a1b2c3d4e5f6/announce, https://home.opsfet.ch/a1b2c3d4e5f6/announce;
* `--content-directory <CONTENT_DIRECTORY>` — Directory containing torrent content. Typically this is set as the download directory in your torrent client. Default: ./content
* `--verbosity <VERBOSITY>` — Level of logs to display. Default: info

  Possible values: `silent`, `error`, `warn`, `info`, `debug`, `trace`

* `--config-path <CONFIG_PATH>` — Path to the configuration file. Default: config.json (in current working directory)
* `--output <OUTPUT>` — Directory where transcodes and spectrograms will be written.

   Default: ./output
* `--target <TARGET>` — Target formats. Default: flac, 320, and v0

  Possible values: `flac`, `320`, `v0`

* `--allow-existing` — Allow transcoding to existing formats
* `--skip-hash-check` — Should the torrent hash check of existing files be skipped?
* `--cpus <CPUS>` — Number of cpus to use for processing. Default: Total number of CPUs
* `--spectrogram-size <SPECTROGRAM_SIZE>` — Output directory to write spectrogram images to

  Possible values: `full`, `zoom`

* `--hard-link` — Use hard links when copying files
* `--compress-images` — Should images greater than the maximum file size be compressed?
* `--max-file-size <MAX_FILE_SIZE>` — Maximum file size in bytes beyond which images are compressed

   Defaults to 750 KB
* `--max-pixel-size <MAX_PIXEL_SIZE>` — Maximum size in pixels for images

   Defaults to 1280 px

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--jpg-quality <JPG_QUALITY>` — Quality percentage to apply for jpg compression.

   Defaults to 80%

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--png-to-jpg` — Should png images be converted to jpg?

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--no-spectrogram` — Should the spectrogram command be executed?
* `--no-upload` — Should the upload command be executed?
* `--limit <LIMIT>` — Limit the number of torrents to batch process
* `--wait-before-upload <WAIT_BEFORE_UPLOAD>` — Wait for a duration before uploading the torrent.

   The duration is a string that can be parsed such as `500ms`, `5m`, `1h30m15s`.
* `--cache <CACHE>` — Path to cache file



## `caesura spectrogram`

Generate spectrograms for each track of a FLAC source

**Usage:** `caesura spectrogram [OPTIONS] [SOURCE]`

###### **Arguments:**

* `<SOURCE>` — Source as: torrent id, path to torrent file, or indexer url.

   Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

###### **Options:**

* `--api-key <API_KEY>` — API key
* `--indexer <INDEXER>` — ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red
* `--indexer-url <INDEXER_URL>` — URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer
* `--announce-url <ANNOUNCE_URL>` — Announce URL including passkey Examples: https://flacsfor.me/a1b2c3d4e5f6/announce, https://home.opsfet.ch/a1b2c3d4e5f6/announce;
* `--content-directory <CONTENT_DIRECTORY>` — Directory containing torrent content. Typically this is set as the download directory in your torrent client. Default: ./content
* `--verbosity <VERBOSITY>` — Level of logs to display. Default: info

  Possible values: `silent`, `error`, `warn`, `info`, `debug`, `trace`

* `--config-path <CONFIG_PATH>` — Path to the configuration file. Default: config.json (in current working directory)
* `--output <OUTPUT>` — Directory where transcodes and spectrograms will be written.

   Default: ./output
* `--spectrogram-size <SPECTROGRAM_SIZE>` — Output directory to write spectrogram images to

  Possible values: `full`, `zoom`

* `--cpus <CPUS>` — Number of cpus to use for processing. Default: Total number of CPUs



## `caesura transcode`

Transcode each track of a FLAC source to the target formats

**Usage:** `caesura transcode [OPTIONS] [SOURCE]`

###### **Arguments:**

* `<SOURCE>` — Source as: torrent id, path to torrent file, or indexer url.

   Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

###### **Options:**

* `--api-key <API_KEY>` — API key
* `--indexer <INDEXER>` — ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red
* `--indexer-url <INDEXER_URL>` — URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer
* `--announce-url <ANNOUNCE_URL>` — Announce URL including passkey Examples: https://flacsfor.me/a1b2c3d4e5f6/announce, https://home.opsfet.ch/a1b2c3d4e5f6/announce;
* `--content-directory <CONTENT_DIRECTORY>` — Directory containing torrent content. Typically this is set as the download directory in your torrent client. Default: ./content
* `--verbosity <VERBOSITY>` — Level of logs to display. Default: info

  Possible values: `silent`, `error`, `warn`, `info`, `debug`, `trace`

* `--config-path <CONFIG_PATH>` — Path to the configuration file. Default: config.json (in current working directory)
* `--output <OUTPUT>` — Directory where transcodes and spectrograms will be written.

   Default: ./output
* `--target <TARGET>` — Target formats. Default: flac, 320, and v0

  Possible values: `flac`, `320`, `v0`

* `--allow-existing` — Allow transcoding to existing formats
* `--hard-link` — Use hard links when copying files
* `--compress-images` — Should images greater than the maximum file size be compressed?
* `--max-file-size <MAX_FILE_SIZE>` — Maximum file size in bytes beyond which images are compressed

   Defaults to 750 KB
* `--max-pixel-size <MAX_PIXEL_SIZE>` — Maximum size in pixels for images

   Defaults to 1280 px

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--jpg-quality <JPG_QUALITY>` — Quality percentage to apply for jpg compression.

   Defaults to 80%

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--png-to-jpg` — Should png images be converted to jpg?

   Only applied if the image is greated than `max_file_size` and `compress_images` is true.
* `--cpus <CPUS>` — Number of cpus to use for processing. Default: Total number of CPUs



## `caesura upload`

Upload transcodes of a FLAC source

**Usage:** `caesura upload [OPTIONS] [SOURCE]`

###### **Arguments:**

* `<SOURCE>` — Source as: torrent id, path to torrent file, or indexer url.

   Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

###### **Options:**

* `--api-key <API_KEY>` — API key
* `--indexer <INDEXER>` — ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red
* `--indexer-url <INDEXER_URL>` — URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer
* `--announce-url <ANNOUNCE_URL>` — Announce URL including passkey Examples: https://flacsfor.me/a1b2c3d4e5f6/announce, https://home.opsfet.ch/a1b2c3d4e5f6/announce;
* `--content-directory <CONTENT_DIRECTORY>` — Directory containing torrent content. Typically this is set as the download directory in your torrent client. Default: ./content
* `--verbosity <VERBOSITY>` — Level of logs to display. Default: info

  Possible values: `silent`, `error`, `warn`, `info`, `debug`, `trace`

* `--config-path <CONFIG_PATH>` — Path to the configuration file. Default: config.json (in current working directory)
* `--output <OUTPUT>` — Directory where transcodes and spectrograms will be written.

   Default: ./output
* `--target <TARGET>` — Target formats. Default: flac, 320, and v0

  Possible values: `flac`, `320`, `v0`

* `--allow-existing` — Allow transcoding to existing formats
* `--copy-transcode-to-content-dir` — Should the transcoded files be copied to the content directory.

   This should be enabled if you wish to auto-add to your torrent client.
* `--copy-torrent-to <COPY_TORRENT_TO>` — Copy the torrent file to the provided directory.

   This should be set if you wish to auto-add to your torrent client.
* `--hard-link` — Use hard links when copying files
* `--dry-run` — Don't upload, just show the data that would be uploaded



## `caesura verify`

Verify a FLAC source is suitable for transcoding

**Usage:** `caesura verify [OPTIONS] [SOURCE]`

###### **Arguments:**

* `<SOURCE>` — Source as: torrent id, path to torrent file, or indexer url.

   Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

###### **Options:**

* `--api-key <API_KEY>` — API key
* `--indexer <INDEXER>` — ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red
* `--indexer-url <INDEXER_URL>` — URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer
* `--announce-url <ANNOUNCE_URL>` — Announce URL including passkey Examples: https://flacsfor.me/a1b2c3d4e5f6/announce, https://home.opsfet.ch/a1b2c3d4e5f6/announce;
* `--content-directory <CONTENT_DIRECTORY>` — Directory containing torrent content. Typically this is set as the download directory in your torrent client. Default: ./content
* `--verbosity <VERBOSITY>` — Level of logs to display. Default: info

  Possible values: `silent`, `error`, `warn`, `info`, `debug`, `trace`

* `--config-path <CONFIG_PATH>` — Path to the configuration file. Default: config.json (in current working directory)
* `--output <OUTPUT>` — Directory where transcodes and spectrograms will be written.

   Default: ./output
* `--target <TARGET>` — Target formats. Default: flac, 320, and v0

  Possible values: `flac`, `320`, `v0`

* `--allow-existing` — Allow transcoding to existing formats
* `--skip-hash-check` — Should the torrent hash check of existing files be skipped?

