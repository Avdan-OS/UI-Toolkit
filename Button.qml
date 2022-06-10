import QtQuick 2.9
import QtQuick.Controls 2.5
import 'qrc:/themeparser.js' as Theme
Item{
    property string text: "Button Text"
    property string background_color:Theme.getValue("button-background-color")
    property string background_color_hover:Theme.getValue("button-background-color::hover")
    property string color:Theme.getValue("button-foreground-color")
    property int radius: Theme.getValue('button-corner-radius')
    property int padding: Theme.getValue('button-padding')
    property int font_size: Theme.getValue('button-font-pixel-size')
    property int horizontal_padding: Theme.getValue('button-padding-horizontal')
    property int vertical_padding: Theme.getValue('button-padding-vertical')
    width:childrenRect.width+padding+horizontal_padding
    height: childrenRect.height+padding+vertical_padding
    MouseArea{
        id:mousearea
        x:0
        y:0
        z:2
        width:parent.width
        height:parent.height
    }
    Rectangle{
        x:0
        id:background
        y:0
        width:parent.width
        radius:parent.radius
        height:parent.height
        z:-1
        color: parent.background_color
    }

    Label{
        color:parent.color
        anchors.centerIn: parent
        text:parent.text
        font.pixelSize: parent.font_size
    } 
}
