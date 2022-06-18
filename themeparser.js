function getValue(id){
    var request = new XMLHttpRequest();
    request.open("GET", `file:///home/adeswanta08/.config/aos/config/theme/main.json`,false);
    request.send(null);
    var json = JSON.parse(String(request.responseText));
    switch (id) {
        case 'window-background-color':
            return String(json["window-background-color"]);
        case 'button-background-color':
            return String(json["button-background-color"]);
        case 'button-background-color::hover':
            return String(json["button-background-color::hover"]);
        case 'button-background-color::pressed':
            return String(json["button-background-color::pressed"]);
        case 'button-foreground-color':
            return String(json["button-foreground-color"]);
        case 'button-corner-radius':
            return String(json["button-corner-radius"]);
        case 'button-padding':
            return String(json["button-padding"]);
        case 'button-font-pixel-size':
            return String(json["button-font-pixel-size"]);
        case 'button-padding-horizontal':
            return String(json["button-padding-horizontal"]);
        case 'button-padding-vertical':
            return String(json["button-padding-vertical"]);
    }
}
