# ezdl

## Introduction
ezdl is a simple Rust program designed to facilitate bulk downloading of files using the Aria2 command-line utility. It allows you to specify a list of URLs along with their corresponding output filenames, and Aria2 will handle the downloading process efficiently.

## Features
- Bulk Downloading: Download multiple files simultaneously from specified URLs.
- Customizable Output: Specify custom output filenames for downloaded files.
- Organized Downloads: Group downloaded files into directories based on user-defined criteria (e.g., year and file name).

## Requirements
1. [rust](https://www.rust-lang.org/) programming language installed.
2. [aria2](https://github.com/aria2/aria2) command-line utility installed and accessible in the system's PATH.

## Usage
1. Ensure Rust and Aria2 are installed on your system.
2. Prepare an input file (`input.txt`) following the specified format:
```php
<Path>
<File Name>
<Filename extension>
<URL 1>
<URL 2>
...
```
3. Building from source and run the program:
```bash
$ git clone https://github.com/vup4m3/ezdl.git
$ cd ezdl
$ cargo run
```

4. The program will read input.txt, organize downloads into directories, and initiate the download process using Aria2.

## Input File Format
The input file (input.txt) should follow this format:
```php
<Path>
<File Name>
<Filename extension>
<URL 1>
<URL 2>
...
```
- `<Path>`: The path in which the files will be organized.
- `<File Name>`:The name of the file or directory to be created for storing the downloaded files.
- `<Filename extension>`: The filename extension.
- `<URL>`:URLs pointing to the files to be downloaded.

Example:
```
2024
Conference_Talks
mp4
https://example.com/talk1.mp4
https://example.com/talk2.mp4
```
## License
ezdl is licensed under the MIT license.[See LICENSE for more information](https://github.com/vup4m3/ezdl/blob/main/LICENSE).
