<div align="center">
    <img 
        src="https://i.postimg.cc/bwmFtxyp/fd-1.png" style="height: 220px; width: 340px;" 
    />
</div>
<p align="center">
    <img src="https://img.shields.io/github/contributors/Atsukoro1/ponyfetch?color=blue&style=for-the-badge"/>
    <img src="https://img.shields.io/github/issues/Atsukoro1/ponyfetch?style=for-the-badge"/>
    <img src="https://img.shields.io/badge/Made%20with-Rust-blue?style=for-the-badge"/>
</p>
<p align="center">
    <b>⚠️ (WIP) This project is not ready for any serious use right now.</b></br></br>
    A cross-platform command-line interface (CLI) tool written in Rust</br> to display system information in an aesthetically pleasing and entertaining manner.
</p>

## 📜 Table of contents

- [📜 Table of contents](#-table-of-contents)
- [✨ Demo](#-demo)
- [🔧 Using](#-using)
- [🧪 Requirements for install](#-requirements-for-install)
- [⚡ Installing](#-installing)
  - [📦 with Cargo](#-with-cargo)
  - [📦 Manually](#-manually)
- [📚 Contributing](#-contributing)
    - [🐎 Adding new ponies](#-adding-new-ponies)
    - [🖥️ Modifying \& Adding code](#️-modifying--adding-code)
- [⛓️ Sources](#️-sources)


## ✨ Demo

`Ponyfetch` provides an aesthetically pleasing, colorful display of important system information, complete with charming ponies.

<img src="https://i.postimg.cc/MK6Fy3tP/konecne.gif" style="height: 300px; width: 570px;" alt="demo">

## 🔧 Using 

```bash
ponyfetch [OPTION]
```

| Option | Description                                              | Type   | Default | Required? |
|--------|----------------------------------------------------------|--------|---------|-----------|
| `-c` or `--color`   | Defines what color to print pony and titles in | `String` | `blue` | No        |
| `-p` or `--pony`   | Pony to print | `String` | `rainbowdash` | No        |
| `-h` or `--help`   | Help menu in case you don't understand this one |  |  | No        |

## 🧪 Requirements for install

On Linux (MAC not tested), install net-tools package using
```sh
sudo apt-get install net-tools
```
or
```sh
sudo pacman -S net-tools
```


## ⚡ Installing

There are multiple possible ways to install Ponyfetch, however, there is still a way to compile it yourself. If you want, please proceed with reading Compiling section.

### 📦 with Cargo
```sh
cargo install ponyfetch
```

### 📦 Manually
To install the latest version of PonyFetch, first visit the available release [here](https://github.com/Atsukoro1/ponyfetch/releases) and download the executable. For Linux, move the binary to the /usr/bin directory. For Windows, move the binary to any desired path and add it to your system's PATH environment variable.

## 📚 Contributing

#### 🐎 Adding new ponies
If you wish to add new ponies to the project, please adhere to the following guidelines in order to maintain the project's structural integrity and functionality. The two main distinguishing features of these ponies are their size and style (e.g. Hat or Wings). Refer to the current pony ASCII art to determine the size of your new pony. If your pony is larger than the others, use the format `<ponyname>_large`. If the size is the same as an existing pony, find a visible difference between your pony and the current one, and name it `<ponyname>_<difference>_large`. If you are still unsure, please open an issue to discuss it with members of our team. Additionally, after adding a new pony, please ensure to credit the author in the "Sources" section of the README.

#### 🖥️ Modifying & Adding code
Ponyfetch is still under development, so it is likely that there are bugs present. If you encounter one, please open an issue and discuss it with us. Once the discussion is completed, submit a pull request with the proposed changes.

## ⛓️ Sources
1. PYTHON, Clive. In: ASCIIMAN [online]. [cit. 2022-12-02]. Available from: https://asciiman.neocities.org/MLP.html
2. My Little Pony ASCII Art. [online]. [cited 2022-12-02]. Available from: https://emojicombos.com/my-little-pony-ascii-art
