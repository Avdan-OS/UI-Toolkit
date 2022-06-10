function getValue(id){
    var request = new XMLHttpRequest();
    request.open("GET", 'file:///home/yegender/.config/aos/config/theme/main.json', false);
    request.send(null);
    var json = JSON.parse(String(request.responseText));
    switch (id){
    case 'window-background-color':
        console.log(json["window-background-color"]);
        return String(json["window-background-color"]);
    }
}
