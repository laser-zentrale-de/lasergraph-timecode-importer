[![Build](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/actions/workflows/build.yml/badge.svg)](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/d-strobel/lasergraph-dsp-timecode-importer/status.svg)](https://deps.rs/repo/github/d-strobel/lasergraph-dsp-timecode-importer)
[![AGPL-3.0 Licensed](https://img.shields.io/github/license/d-strobel/lasergraph-dsp-timecode-importer)](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/blob/main/LICENSE)

# Lasergraph DSP timecode importer
Timecode importer for the Laseranimation Sollinger Lasergraph DSP.

## Description
This tool imports timestamps from a csv-file to the LaserAnimation Sollinger Lasergraph DSP timescript and adds the corresponding entries.<br>

⚠️ **IMPORTANT**: This project is not related to LaserAnimation Sollinger GmbH. When using this command line tool, please report any bugs or suggestions directly via Github Issues.

## Features
- Import timestamps to timescript
- Import entrys to film
- Set start number for first entry
- Import from CSV file

## Usage
#### CLI (Linux)
```bash
# Download binary from github
chmod +x ./lasergraph-dsp-timecode-importer
./lasergraph-dsp-timecode-importer --address <IP address of your dsp> --csv <path to the csv file>
```
#### CLI (MacOS)
```bash
# Download binary from github
# Coming soon
```
#### CLI (Windows)
```bash
# Download binary from github
# Coming soon
```

## Why?
Sometimes it's hard to find the correct timestamp for specific (out of beat) sequences in the Lasergraph DSP Timescript.<br>
There are thrid party tools like Reaper, where you can set specific markers to the the timeline and export them afterwards to a csv-file.<br>
For light shows with the MaLighting GrandMa2, it is common practice to use the exported csv and import it to the consol.<br>
<br>
I want to adapt this workflow to the Lasergraph DSP.
