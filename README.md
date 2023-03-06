[![build](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/actions/workflows/build.yml/badge.svg)](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/d-strobel/lasergraph-dsp-timecode-importer/status.svg)](https://deps.rs/repo/github/d-strobel/lasergraph-dsp-timecode-importer)
[![AGPL-3.0 Licensed](https://img.shields.io/github/license/d-strobel/lasergraph-dsp-timecode-importer)](https://github.com/d-strobel/lasergraph-dsp-timecode-importer/blob/main/LICENSE)

# Lasergraph DSP timecode importer
Timecode importer for the Laseranimation Sollinger Lasergraph DSP.

## Description
This tool imports timestamps from a csv-file to the LaserAnimation Sollinger Lasergraph DSP timescript and optionally adds the corresponding entries.<br>

⚠️ **IMPORTANT**: This project is not related to LaserAnimation Sollinger GmbH. When using this command line tool, please report any bugs or suggestions directly via Github Issues.

## Features
- Import timestamps to timescript
- Import entrys to film (optional)
- Set start number for first entry (optional)
- Import from CSV file

## Usage
```
Usage: lasergraph-dsp-timecode-importer [OPTIONS] --address <IP-ADDRESS> --csv <FILE>

Options:
  -a, --address <IP-ADDRESS>  IP-Address of the Lasergraph DSP
  -p, --port <PORT>           TCP/IP port of the lasergraph DSP for remoting [default: 8210]
  -c, --csv <FILE>            Path to the CSV-file
  -e, --create-entry          Defines if entries should be created
  -s, --start-entry <ENTRY>   Defines the number of the first Entry [default: 0]
  -h, --help                  Print help
  -V, --version               Print version
```

## Why?
Sometimes it's hard to find the correct timestamp for specific (out of beat) sequences in the Lasergraph DSP Timescript.<br>
There are thrid party tools like Reaper, where you can set specific markers to the the timeline and export them afterwards to a csv-file.<br>
For light shows with the MaLighting GrandMa2, it is common practice to use the exported csv and import it to the consol.<br>
<br>
I want to adapt this workflow to the Lasergraph DSP.
