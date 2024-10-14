# <p style="text-align: center">caesura 𝄓</p>

An all-in-one command line tool to automate transcoding FLAC or FLAC 24 bit source torrent to MP3 320 (CBR) and MP3 V0 (VBR), then upload to gazelle based trackers.

## Features

All gazelle based indexers/trackers are supported
- RED
- **[[wip](https://github.com/RogueOneEcho/caesura/issues/7)]** OPS.

Tested on Linux, theoretically works on Windows.

Fully configurable, if there's something hard coded that you think should be configurable then open issue on GitHub.

### Source Verification

Each source is verified to ensure:
- A lossless FLAC
- Not a scene or lossy release
- Files match the torrent hash
- Audio tags for artist, album, title and track number are set
- **[[fixed]](https://github.com/RogueOneEcho/caesura/issues/47)]** Classical sources have a composer tag.
- **[[fixed](https://github.com/RogueOneEcho/caesura/issues/18)]** Vinyl track numbering
- Sample rate and channels are suitable

### Spectrogram Generation

- Full and zoomed spectrograms generated for review

### Transcoding

- **[fixed]** Multi-threaded transcoding with optional CPU limit
- FLAC and FLAC 24 bit sources are supported
- FLAC, MP3 320 (CBR) and MP3 V0 (VBR) target formats
- Existing formats are skipped
- **[[fixed](https://github.com/RogueOneEcho/caesura/issues/21)]** Nested sub directories are fully supported (i.e. CD1, and CD2 etc)
- **[[fixed](https://github.com/RogueOneEcho/caesura/issues/22)]** Automatic naming following established conventions, with decoding of HTML entities.
- **[[fixed](https://github.com/RogueOneEcho/caesura/issues/24)]** Shorter file names.
- Automatic torrent file creation
- **[new]** Images in the root and first nested directory are included and all other files ignored.
- **[new]** Images larger than 750 KB are reduced to less than 1280 px, converted to JPG and compressed.

*The logic being that for transcodes only folder and cover images are important. Anyone interested in additional files and high quality artwork can find them in the source torrent.*

### Upload

- Copy transcodes to content directory
- Copy torrent file to client auto-add directory

### Batch

- **[new]** Verify, transcode and upload with one command for every torrent file in a directory.

*The application will crunch through your torrent directory and automatically determine which are FLAC sources suitable for transcoding.*

## Getting started

Docker is the recommended way to run the application across all platforms.
- All dependencies are built into the image
- Runs in an isolated environment reducing risks to your system

> [!TIP]
> **[Configuration options and the commands they apply to are documented in COMMANDS.md](COMMANDS.md)**

### 0. Install Docker

[Install Docker Engine](https://docs.docker.com/engine/install/) for your OS.

### 1. Run the `help` command

Run the `help` command to see the available commands and options.

```bash
docker run ghcr.io/rogueoneecho/caesura --help
```

> [!TIP]
> You can append `--help` to any command to see the available options.
>
> ```bash
> docker run ghcr.io/rogueoneecho/caesura verify --help
> ```

### 2. Create a configuration file

Run the `config` command to print the default configuration and redirect it to `config.json`.

```bash
docker run ghcr.io/rogueoneecho/caesura config > config.json
```

> [!NOTE]
> You can ignore the "Failed to read config file" warning.

Edit `config.json` in your preferred text editor. Set the following fields for your indexer:
- `announce_url` Your personal announce URL. Find it on upload page.
- `api_key` Create an API key with `Torrents` permission `Settings > Access Settings > Create an API Key`
- `content` the directory containing torrent content. Typically this is set as the download directory in your torrent client. Defaults to `./content`.
- `indexer` the id of the indexer: `red`, `pth`, `ops`.
- `indexer_url` the URL of the indexer: `https://redacted.ch`, `https://orpheus.network`.
- `output` the directory where transcodes, spectrograms and .torrent files will be written. Defaults to `./output`.

### 3. Verify a source

Run the `verify` command with the source as an argument.

> [!NOTE]
> Because the application is running in a Docker container, you need to mount the config file, content directory and output directory.

> [!TIP]
> For the source you can use a permalink, the numeric torrent id or a path to a torrent file:
>
> Each step of this guide will use a different source to demonstrate, but feel free to use whichever suits you best.

```bash
docker run \
-v ./config.json:/config.json \
-v /path/to/your/content:/content \
-v ./output:/output \
ghcr.io/rogueoneecho/caesura \
verify https://redacted.ch/torrents.php?id=80518&torrentid=142659#torrent142659
```

If it looks good you can proceed to the next step, otherwise try another source.

### 4. Use Docker Compose

Docker is great but specifying the volumes everytime is tedious and prone to error.

Using Docker Compose simplifies this by storing the configuration in a `docker-compose.yml` file.

Create a `docker-compose.yml` file with the following content:

```yaml
services:
  caesura:
    container_name: caesura
    image: ghcr.io/rogueoneecho/caesura
    volumes:
    - ./config.json:/config.json:ro
    - /path/to/your/content:/content:ro
    - ./output:/output
```

> [!NOTE]
> The `:ro` suffix makes the volume read-only which is a good security practice.
>
> If you intend to use the `--copy-transcode-to-content-dir` option then you must remove the `:ro` suffix from the content volume.

Now run the verify command again but this time using Docker Compose:

```bash
docker compose run --rm caesura verify 142659
```

### 5. Generate spectrograms of a source

Run the `spectrogram` command with the source as an argument.

```bash
docker compose run --rm caesura spectrogram 142659
```

Inspect the spectrograms in the output directory.

### 6. Transcode a source

Run the `transcode` command with the source as an argument.

```bash
docker compose run --rm caesura transcode "Khotin - Hello World [2014].torrent"
```

Inspect the transcodes in the output directory.

> [!TIP]
> Things to check:
> - Folder structure
> - File names
> - Tags
> - Audio quality
> - Image size and compression quality

### 7. Upload transcodes

> [!WARNING]
> You are responsible for everything you upload.
>
> Misuse of this application can result in the loss of your upload privileges.

Run the `upload` command with the source as an argument.

> [!TIP]
> Ideally you've already checked everything and nothing will go wrong but just in case there is a grace period after uploading in which you can remove the upload from your indexer.

> [!TIP]
> If you're unsure about this then you can append `--dry-run` to the command and instead of uploading it will print the data that would be submitted.

```bash
docker compose run --rm caesura upload https://redacted.ch/torrents.php?id=80518&torrentid=142659#torrent142659
```

Go to your indexer and check your uploads to make sure everything has gone to plan.

### 8. Batch processing

> [!WARNING]
> You are responsible for everything you upload.
>
> Misuse of this application, especially the `batch` command, can result in the loss of your upload privileges or a ban.

Now that you have the hang of the application we can speed things up with the `batch` command.

This handles `verify`, `spectrogram`, `transcode` and `upload` in a single command. It can also be pointed at a directory containing torrent files to automatically sort through and pick out suitable sources.

By default the `batch` command will limit to processing just `3` transcodes and it won't create spectrograms or upload unless explicitly instructed. These safeguards are in place to prevent mistakenly uploading a bunch of sources that you haven't checked.

> [!NOTE]
> The batch command uses a cache file to store progress helping speed up subsequent runs.
>
> Make sure the cache file is in a mounted volume so it's not deleted between runs.

Run the command to compile a cache and transcode the first three sources in the directory:

```bash
docker compose run --rm caesura batch /path/to/your/torrents
```

> [!TIP]
> Add the `--spectrogram` flag to generate spectrograms.

If everything goes to plan three sources should have transcoded to your output directory.

You can filter the cache file with `jq` to see what has been transcoded:

```bash
cat ./output/cache.json | jq 'map(select(.transcoded == true))'
```

Or to see what has been skipped and why:

```bash
cat ./output/cache.json | jq 'map(select(.skipped != null))'
```

If you're working with a lot of files then `less` can be helpful:

```bash
cat ./output/cache.json | jq --color-output 'map(select(.transcoded == true))' | less -R
```

Nothing was uploaded in the first run giving you a chance to check the transcodes and spectrograms. Once you're satisfied run the command again but with the `--upload` flag (or set `"upload": true` in the config file).

```bash
docker compose run --rm caesura batch /path/to/your/torrents --upload
```

Check the uploads on your indexer to make sure everything has gone to plan.

Now, we can set the batch command loose with the `--no-limit` option to transcode (but not upload) every source in the directory:

```bash
docker compose run --rm caesura batch /path/to/your/torrents --no-limit
```

Once you've checked the transcodes you can start to upload them in batches. The `--wait-before-upload 30s` option will add a 30 second wait interval between uploads to give you time to check everything looks good, and spread out the load on your indexer:

```bash
docker compose run --rm caesura batch /path/to/your/torrents --upload --limit 10 --wait-before-upload 30s
```

> [!WARNING]
> In theory you can execute with both `--upload --no-limit` but that is probably a bad idea and a very fast way to lose your upload privileges.
>
> If you are going to do so then you should definitely use a long wait interval:
> `--upload --no-limit --wait-before-upload 2m`

## Commands and Configuration

> [!TIP]
> **[Configuration options and the commands they apply to are documented in COMMANDS.md](COMMANDS.md)**

Configuration options are sourced first from the command line arguments, then from a configuration file.

By default the application loads `config.json` from the current working directory, but this can be overridden with the `--config <CONFIG_PATH>` cli argument.

Most options have sensible defaults so the minimum required configuration is:

```json
{
    "announce_url": "https://flacsfor.me/YOUR_ANNOUNCE_KEY/announce",
    "api_key": "YOUR_API_KEY",
}
```

### Recommended configuration

This is based around the setup in this guide: [how to set up Deluge via Proton VPN with port forwarding](https://github.com/RogueOneEcho/how-to-setup-deluge-with-protonvpn-portforward).

#### Directory structure

- `/srv/shared` is a shared between multiple containers, by mounting as a single volume hard linking is possible.
- `/srv/deluge/state` is the Deluge state directory, containing all `.torrent` files loaded in Deluge.
- `/srv/shared/deluge` is the Deluge download directory, containing all the content.

#### `config.json`

- `"source": "/srv/deluge/state",` in `config.json` means the source can be ommitted from the command.

```json
{
    "announce_url": "https://flacsfor.me/YOUR_ANNOUNCE_KEY/announce",
    "api_key": "YOUR_API_KEY",
    "cache": "/srv/shared/caesura/cache.json",
    "content": "/srv/shared/deluge",
    "limit": 5,
    "output": "/srv/shared/caesura",
    "source": "/srv/deluge/state",
    "verbosity": "debug"
}
```

#### `docker-compose.yml`

- `user: "1000:1001"` ensures files have the same ownership as the host user (use the `id` command to find your user and group id).
- Only `/srv/shared` has write permissions, the other directories are read-only.
- `command: batch` runs the batch command by default.
- `/` is the working directory of the container so mounting the config to `/config.json` means it's read by default.

```yaml
services:

  caesura:
    container_name: caesura
    image: ghcr.io/rogueoneecho/caesura
    user: "1000:1001"
    volumes:
    - /srv/caesura/config.json:/config.json:ro
    - /srv/deluge/state:/srv/deluge/state:ro
    - /srv/shared:/srv/shared
```

## Troubleshooting

If you encounter any issues:

1. Check the logs for errors.

The logging verbosity can be adjusted with the `--verbosity <LOG-LEVEL>` option. The available log levels are:

- `warn` only showing warnings and errors
- `info` will give an overview of what's happening
- `debug` provides insight into each step
- `trace` is detailed logging to see exactly what's happening

2. Re-read the getting started guide
3. [Ask for help in GitHub Discussions](https://github.com/RogueOneEcho/caesura/discussions)
4. [Create an issue](https://github.com/RogueOneEcho/caesura/issues)

> [!TIP]
> If you manage to resolve your issue it's always worth creating a new discussion anyway because it might help someone else in the future, or identify an area where the documentation could be improved.

## Build

**[The build process is documented in BUILD.md](BUILD.md)**

## Releases and Changes

Releases and a full changelog are available via [GitHub Releases](https://github.com/RogueOneEcho/caesura/releases).

Release versions follow the [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) specification.

Commit messages follow the [Conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) specification.

## History

[**DevYukine**](https://github.com/DevYukine) completed the **initial work** and released it as [**red_oxide**](https://github.com/DevYukine/red_oxide) under an [MIT license](LICENSE.HISTORIC.md).

[**RogueOneEcho**](https://github.com/RogueOneEcho) forked the project to complete a major refactor, **fix some issues**, add **new features** and improve logging and error handling. The fork is released as [**caesura**](https://github.com/RogueOneEcho/caesura) under an [AGPL license](LICENSE.md).

*The main difference between the former MIT license and the present AGPL license is that if you intend to distribute a modified version of the code - even to run it on a server - you must also provide the modified source code under an AGPL license.*

*This is often known as copyleft. The intent is to ensure that anyone taking advantage of this open source work are also contributing back to the open source community.*

The code base has now adopted [object oriented patterns](https://refactoring.guru/design-patterns/catalog) with SOLID principles and [dependency injection](https://en.wikipedia.org/wiki/Dependency_injection).

See also the list of
[contributors](https://github.com/RogueOneEcho/caesura/contributors)
who participated in this project.
