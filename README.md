# UI-Toolkit

AvdanOS UI Toolkit made with egui

## Screenshots

Light Mode
![Screenshot1](screenshots/light.png)

Dark Mode
![Screenshot1](screenshots/dark.png)

## How to use this

### Step 1: Setup

On debian-based linux distros run this command on the terminal:

```sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev```

Meanwhile on Fedora-based distros, you need to run this command:

```sudo dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel```

### Step 2: Building and running

1. Clone this repository : (`git clone https://github.com/Avdan-OS/UI-Toolkit.git`)

2. Build the cloned repo : (`cd UI-Toolkit && cargo run`)

3. Yay! You're done!

### Useful Resources

Explore the different eGUI Widgets : `https://www.egui.rs/index.html#demo`
