<div align="center">
<div>
  
```
  ____        _       ____  _           _   
 | __ ) _   _| |_ ___| __ )| | __ _ ___| |_ 
 |  _ \| | | | __/ _ \  _ \| |/ _` / __| __|
 | |_) | |_| | ||  __/ |_) | | (_| \__ \ |_ 
 |____/ \__, |\__\___|____/|_|\__,_|___/\__|
        |___/                               
```
</div>

# ByteBlast
## Easily create dummy files for various purposes

![github stars badge](https://badgen.net/github/stars/samuelchristlie/ByteBlast?icon=github)
![github forks badge](https://badgen.net/github/forks/samuelchristlie/ByteBlast?icon=github)
![github issues badge](https://badgen.net/github/open-issues/samuelchristlie/ByteBlast?icon=github)

![github last commit badge](https://badgen.net/github/last-commit/samuelchristlie/ByteBlast?icon=github)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

</div>

**ByteBlast** is a versatile dummy file creator written in Rust. It allows you to generate large files filled with random byte patterns, making it useful for various testing scenarios, such as performance benchmarking, stress testing, or simulating realistic file sizes.

## Features 💪
- **Efficient and Fast**: ByteBlast is built using Rust, a high-performance systems programming language, ensuring efficient file generation even for extremely large files.
- **Flexible File Generation**: ByteBlast provides a simple and intuitive command-line interface that enables you to generate files of any size, with customizable byte patterns.
- **Random Byte Generation**: Generate files with random byte patterns, useful for testing scenarios where data integrity or randomness is important.

## Table of Contents 📝
1. 💻 [Installation](#installation)
2. ▶ [Usage](#usage)
3. 📃 [License](#license)
4. 🙏 [Acknowledgements](#acknowledgements)

<a name="installation"/>

## 💻 Installation
### Building
Clone this repository
```
git clone https://github.com/samuelchristlie/ByteBlast
cd ByteBlast
```
Build project using Cargo
```
cargo build --release
```
The compiled build should be located in the target folder.
### Prebuilt Binary
You can also check if a prebuilt binary is available in the [Releases](https://github.com/samuelchristlie/ByteBlast/releases) section.


<a name="usage"/>

## ▶ Usage
To run **ByteBlast**, simply type the following command
```
byteblast
```
<a name="license"/>

## 📃 License
This project is licensed under the GNU GPL v3 License - see the [LICENSE](LICENSE) file for more details.

<a name="acknowledgements"/>

## 🙏 Acknowledgements
Thanks to Patrick Gillespie for creating the ASCII text art tool used in this project
https://patorjk.com/software/taag/
