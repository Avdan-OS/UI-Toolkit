# UI-Toolkit

AvdanOS UI Toolkit made with Rust using the iced Library.

## Welp I can't get it running at first compile!

Yeah, me too... Before compiling make sure you have these two packages installed:
`cmake` & `fontconfig` (oh, and obviously don't forget Rust ;)

On Ubuntu:

```
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install cmake libfontconfig-dev1
```

On Fedora:

```
sudo dnf update
sudo dnf install cmake fontconfig-devel
```

On Arch:

```
sudo pacman -Syu
sudo pacman -S cmake lib32-fontconfig
```

On Void:

```
sudo xbps-install -Su
sudo xbps-install cmake fontconfig-devel
```
