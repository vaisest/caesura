# <p style="text-align: center">caesura 𝄓</p>

An all-in-one command line tool to automate transcoding FLAC or FLAC 24 bit source torrent to MP3 320 (CBR) and MP3 V0 (VBR), then upload to gazelle based trackers.

## Features

All gazelle based indexers/trackers are supported
- RED
- **[[wip](https://github.com/RogueOneEcho/caesura/issues/7)]** OPS.

Tested on Linux, theoretically works on Windows.

Fully configurable, if there's something hard coded that you think should be configurable then open issue on GitHub.

### Source Verification

Each source is verified to ensure it's:
- A lossless FLAC
- Not a scene or lossy release
- Files match the torrent hash
- Audio tags for artist, album, title and track number are set
- **[[fixed](https://github.com/DevYukine/red_oxide/issues/18)]** Vinyl track numbering
- Sample rate and channels are supported

### Spectrogram Generation

- Full and zoomed spectrograms generated for review

### Transcoding

- **[fixed]** Multi-threaded transcoding with optional CPU limit
- FLAC and FLAC 24 bit sources are supported
- FLAC, MP3 320 (CBR) and MP3 V0 (VBR) target formats
- Existing formats are skipped
- **[[fixed](https://github.com/DevYukine/red_oxide/issues/21)]** Nested sub directories are fully supported (i.e. CD1, and CD2 etc)
- **[[fixed](https://github.com/DevYukine/red_oxide/issues/22)]** Automatic naming following established conventions, with decoding of HTML entities.
- **[[fixed](https://github.com/DevYukine/red_oxide/issues/24)]** Shorter file names.
- Automatic torrent file creation
- **[new]** Images in the root directory are included and all other files ignored.
- **[new]** Images larger than 750 KB are (optionally) compressed, converted to JPG and reduced to less than 1920 px. 

*The logic being that folder and cover images are included but to minimize file size, but for artwork and anything additional the original source can be downloaded*

### Upload

- Copy transcodes to content directory
- Copy torrent file to client auto-add directory

### Batch

- **[new]** Verify, transcode and upload from each torrent file in a directory. 

*The application will crunch through your torrent directory and automatically determine which are FLAC sources suitable for transcoding.*

## Getting started

The quickest path to get running is using docker compose, but that will require a little familiarity with docker. But the benefits are reproducability and peace of mind through isolation. 

### Install

#### Docker

1. Build

```bash
docker build -t caesura .
```

2. Run

```bash
docker run \
--user 1000:1000 \
-v ./path/to/your/config.json:/config.json \
-v /path/to/your/data:/data \
caesura verify "123456"
```

#### Docker Compose

```bash
docker compose run --rm caesura verify "123456"
```

#### Linux and MacOS

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. [Install Homebrew](https://brew.sh/)

3. [Install Intermodal](https://github.com/casey/intermodal#installation)

```bash
cargo install imdl
```

4. Install FLAC, LAME, SOX and ImageMagick dependencies.

```bash
brew install flac lame sox imagemagick
```

5. Install caesura
```bash
cargo install caesura
```

#### Ubuntu

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. [Install Intermodal](https://github.com/casey/intermodal#installation)

```bash
cargo install imdl
```

3. Install FLAC, LAME, SOX and ImageMagick dependencies.

```bash
sudo apt install flac lame sox imagemagick --yes
```

4. Install caesura
```bash
cargo install caesura
```

#### Windows

*To be confirmed*

For now it's recommended to run with docker.

### CLI Commands

#### Verify source

Verify a FLAC source is suitable for transcoding.

```
caesura verify [OPTIONS] [SOURCE]
```

<details>
<summary><code>caesura verify --help</code></summary>

```
Usage: caesura verify [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]
          Source as: torrent id, path to torrent file, or indexer url.
          
          Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

Options:
      --api-key <API_KEY>
          API key

      --indexer <INDEXER>
          ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red

      --indexer-url <INDEXER_URL>
          URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer

      --tracker-url <TRACKER_URL>
          URL of the tracker. Examples: https://flacsfor.me, https://home.opsfet.ch; Default: Dependent on indexer

      --content-directory <CONTENT_DIRECTORY>
          Directory containing torrent content. Typically this is set as the download directory in your torrent client

      --cpus <CPUS>
          Number of cpus to use for processing. Default: Total number of CPUs

      --verbosity <VERBOSITY>
          Level of logs to display. Default: info
          
          [possible values: silent, error, warn, info, debug, trace]

      --config-path <CONFIG_PATH>
          Path to the configuration file. Default: config.json (in current working directory)

      --output <OUTPUT>
          Directory where transcodes and spectrograms will be written

      --target <TARGET>
          Target formats. Default: flac, 320, and v0
          
          [possible values: flac, 320, v0]

      --allow-existing
          Allow transcoding to existing formats

      --skip-hash-check
          Should the torrent hash check of existing files be skipped?

      --hard-link
          Use hard links when copying files

      --compress-images
          Should images greater than 750 KB be compressed?

  -h, --help
          Print help (see a summary with '-h')
```
</details>

#### Generate spectrograms

Generate spectrograms for each track of a FLAC source.

```
caesura spectrogram [OPTIONS] [SOURCE]
```

<details>
<summary><code>caesura spectrogram --help</code></summary>

```
Usage: caesura spectrogram [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]
          Source as: torrent id, path to torrent file, or indexer url.
          
          Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

Options:
      --api-key <API_KEY>
          API key

      --indexer <INDEXER>
          ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red

      --indexer-url <INDEXER_URL>
          URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer

      --tracker-url <TRACKER_URL>
          URL of the tracker. Examples: https://flacsfor.me, https://home.opsfet.ch; Default: Dependent on indexer

      --content-directory <CONTENT_DIRECTORY>
          Directory containing torrent content. Typically this is set as the download directory in your torrent client

      --cpus <CPUS>
          Number of cpus to use for processing. Default: Total number of CPUs

      --verbosity <VERBOSITY>
          Level of logs to display. Default: info
          
          [possible values: silent, error, warn, info, debug, trace]

      --config-path <CONFIG_PATH>
          Path to the configuration file. Default: config.json (in current working directory)

      --output <OUTPUT>
          Directory where transcodes and spectrograms will be written

      --spectrogram-size <SPECTROGRAM_SIZE>
          Output directory to write spectrogram images to
          
          [possible values: full, zoom]

  -h, --help
          Print help (see a summary with '-h')
```

</details>

#### Transcode FLACs

Transcode each track of a FLAC source to the target formats.

```
caesura transcode [OPTIONS] [SOURCE]
```

<details>
<summary><code>caesura transcode --help</code></summary>

```
Usage: caesura transcode [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]
          Source as: torrent id, path to torrent file, or indexer url.
          
          Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

Options:
      --api-key <API_KEY>
          API key

      --indexer <INDEXER>
          ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red

      --indexer-url <INDEXER_URL>
          URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer

      --tracker-url <TRACKER_URL>
          URL of the tracker. Examples: https://flacsfor.me, https://home.opsfet.ch; Default: Dependent on indexer

      --content-directory <CONTENT_DIRECTORY>
          Directory containing torrent content. Typically this is set as the download directory in your torrent client

      --verbosity <VERBOSITY>
          Level of logs to display. Default: info
          
          [possible values: silent, error, warn, info, debug, trace]

      --config-path <CONFIG_PATH>
          Path to the configuration file. Default: config.json (in current working directory)

      --output <OUTPUT>
          Directory where transcodes and spectrograms will be written

      --target <TARGET>
          Target formats. Default: flac, 320, and v0
          
          [possible values: flac, 320, v0]

      --allow-existing
          Allow transcoding to existing formats

      --hard-link
          Use hard links when copying files

      --compress-images
          Should images greater than the maximum file size be compressed?

      --max-file-size <MAX_FILE_SIZE>
          Maximum file size in bytes beyond which images are compressed
          
          Defaults to 750 KB

      --max-pixel-size <MAX_PIXEL_SIZE>
          Maximum size in pixels for images
          
          Defaults to 1280 px
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --jpg-quality <JPG_QUALITY>
          Quality percentage to apply for jpg compression.
          
          Defaults to 80%
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --png-to-jpg
          Should png images be converted to jpg?
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --cpus <CPUS>
          Number of cpus to use for processing. Default: Total number of CPUs

  -h, --help
          Print help (see a summary with '-h')
```
</details>

#### Upload

Upload transcodes of a FLAC source.

```
caesura upload [OPTIONS] [SOURCE]
```

<details>
<summary><code>caesura upload --help</code></summary>

```
Usage: caesura upload [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]
          Source as: torrent id, path to torrent file, or indexer url.
          
          Examples: 4871992, path/to/something.torrent, https://example.com/torrents.php?id=2259978&torrentid=4871992#torrent4871992, or https://example.com/torrents.php?torrentid=4871992

Options:
      --api-key <API_KEY>
          API key

      --indexer <INDEXER>
          ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red

      --indexer-url <INDEXER_URL>
          URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer

      --tracker-url <TRACKER_URL>
          URL of the tracker. Examples: https://flacsfor.me, https://home.opsfet.ch; Default: Dependent on indexer

      --content-directory <CONTENT_DIRECTORY>
          Directory containing torrent content. Typically this is set as the download directory in your torrent client

      --verbosity <VERBOSITY>
          Level of logs to display. Default: info
          
          [possible values: silent, error, warn, info, debug, trace]

      --config-path <CONFIG_PATH>
          Path to the configuration file. Default: config.json (in current working directory)

      --output <OUTPUT>
          Directory where transcodes and spectrograms will be written

      --target <TARGET>
          Target formats. Default: flac, 320, and v0
          
          [possible values: flac, 320, v0]

      --allow-existing
          Allow transcoding to existing formats

      --copy-transcode-to-content-dir
          Should the transcoded files be copied to the content directory.
          
          This should be enabled if you wish to auto-add to your torrent client.

      --copy-torrent-to <COPY_TORRENT_TO>
          Copy the torrent file to the provided directory.
          
          This should be set if you wish to auto-add to your torrent client.

      --hard-link
          Use hard links when copying files

  -h, --help
          Print help (see a summary with '-h')
```
</details>

#### Batch

Verify, transcode, and upload from multiple FLAC sources in one command.

```
caesura batch [OPTIONS] [SOURCE]
```

<details>
<summary><code>caesura batch --help</code></summary>

```
Usage: caesura batch [OPTIONS] [SOURCE]

Arguments:
[SOURCE]
Source as: path to directory of torrents.

Options:
      --api-key <API_KEY>
          API key

      --indexer <INDEXER>
          ID of the tracker as it appears in the source field of a torrent. Examples: red, pth, ops; Default: red

      --indexer-url <INDEXER_URL>
          URL of the indexer. Examples: https://redacted.ch, https://orpheus.network; Default: Dependent on indexer

      --tracker-url <TRACKER_URL>
          URL of the tracker. Examples: https://flacsfor.me, https://home.opsfet.ch; Default: Dependent on indexer

      --content-directory <CONTENT_DIRECTORY>
          Directory containing torrent content. Typically this is set as the download directory in your torrent client

      --verbosity <VERBOSITY>
          Level of logs to display. Default: info
          
          [possible values: silent, error, warn, info, debug, trace]

      --config-path <CONFIG_PATH>
          Path to the configuration file. Default: config.json (in current working directory)

      --output <OUTPUT>
          Directory where transcodes and spectrograms will be written

      --target <TARGET>
          Target formats. Default: flac, 320, and v0
          
          [possible values: flac, 320, v0]

      --allow-existing
          Allow transcoding to existing formats

      --skip-hash-check
          Should the torrent hash check of existing files be skipped?

      --cpus <CPUS>
          Number of cpus to use for processing. Default: Total number of CPUs

      --spectrogram-size <SPECTROGRAM_SIZE>
          Output directory to write spectrogram images to
          
          [possible values: full, zoom]

      --hard-link
          Use hard links when copying files

      --compress-images
          Should images greater than the maximum file size be compressed?

      --max-file-size <MAX_FILE_SIZE>
          Maximum file size in bytes beyond which images are compressed
          
          Defaults to 750 KB

      --max-pixel-size <MAX_PIXEL_SIZE>
          Maximum size in pixels for images
          
          Defaults to 1280 px
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --jpg-quality <JPG_QUALITY>
          Quality percentage to apply for jpg compression.
          
          Defaults to 80%
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --png-to-jpg
          Should png images be converted to jpg?
          
          Only applied if the image is greated than `max_file_size` and `compress_images` is true.

      --no-spectrogram
          Should the spectrogram command be executed?

      --no-upload
          Should the upload command be executed?

      --limit <LIMIT>
          Limit the number of torrents to batch process

      --wait-before-upload <WAIT_BEFORE_UPLOAD>
          Wait for a duration before uploading the torrent.
          
          The duration is a string that can be parsed such as `500ms`, `5m`, `1h30m15s`.

      --cache <CACHE>
          Path to cache file

  -h, --help
          Print help (see a summary with '-h')
```
</details>

### Configuration

Configuration options are sourced first from the command line arguments, then from a configuration file.

By default the application loads `config.json` from the current working directory, but this can be overridden with the `--config-path <CONFIG_PATH>` cli argument.

Most options have sensible defaults so the minimum required configuration is:

```json
{
    "api_key": "YOUR_API_KEY",
    "output": "path/to/write/output",
    "content_directory": "path/to/your/torrent/content_or_downloads"
}
```

Full configuration:

```json
{
    "api_key": "YOUR_API_KEY",
    "indexer": "abc",
    "indexer_url": "https://example.com",
    "tracker_url": "https://tracker.example.com",
    "content_directory": "path/to/your/torrent/content_or_downloads",
    "cpus": 6,
    "verbosity": "trace",
    "output": "path/to/write/output",

    "target": ["320", "v0", "flac"],
    "allow_existing": false,
    "skip_hash_check": false,
    "hard_link": false,
    "compress_images": false,
    
    "spectrogram_size": ["full", "zoom"],

    "copy_transcode_to_content_dir": false,
    "copy_torrent_to": "path/to/copy/torrent",
    "hard_link": false,
    
    "source": "123456"
}


```

## Releases and Changes

Release versions follow the [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) specification.

Commit messages follow the [Conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) specification.

Releases and a full changelog are available via [GitHub Releases](https://github.com/RogueOneEcho/caesura/releases).

## History

[**DevYukine**](https://github.com/DevYukine) completed the **initial work** and released it as [**red_oxide**](https://github.com/DevYukine/red_oxide) under an [MIT license](LICENSE.HISTORIC.md).

[**RogueOneEcho**](https://github.com/RogueOneEcho) forked the project to complete a major refactor, **fix some issues**, add **new features** and improve logging and error handling. The fork is released as [**caesura**](https://github.com/RogueOneEcho/caesura) under an [AGPL license](LICENSE.md).

*The main difference between the former MIT license and the present AGPL license is that if you intend to distribute a modified version of the code - even to run it on a server - you must also provide the modified source code under an AGPL license.*

*This is often known as copyleft. The intent is to ensure that anyone taking advantage of this open source work are also contributing back to the open source community.*

The code base has now adopted [object oriented patterns](https://refactoring.guru/design-patterns/catalog) with SOLID principles and [dependency injection](https://en.wikipedia.org/wiki/Dependency_injection).

See also the list of
[contributors](https://github.com/DevYukine/red_oxide/contributors)
who participated in this project.
