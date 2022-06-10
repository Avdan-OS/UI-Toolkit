# avdan-qml-toolkit
Avdan OS UI Toolkit(Just a QML Library or Module)

# Theme Parser Engine
This thing will read user theme config file. So, that if a user changes the config file then everyapp use config got theme update

Currently it supports only one key from config that is window-background-color.

## Known Bugs of Theme Parser Engine
1. Only works for me because of my username, you have to change it
2. Only one key is accessible
3. Needs to restart when theme is changed

# Config File
It's a json file. It should be present at `/home/`[username 'only yegender works for now by defaukt']`/.config/aos/config/theme/main.json`

# Sample Config
```
{
        "window-background-color":"#00000000"
}
```
