import QtQuick 2.9
import QtQuick.Controls 2.5
import 'qrc:/themeparser.js' as Theme

Item{
    property string text: "Button Text"
    property string background_color: Theme.getValue("button-background-color")
    property string background_color_hover: Theme.getValue("button-background-color::hover")
    property string background_color_pressed: Theme.getValue("button-background-color::pressed")
    property string color: Theme.getValue("button-foreground-color")
    property int radius: Theme.getValue('button-corner-radius')
    property int padding: Theme.getValue('button-padding')
    property int font_size: Theme.getValue('button-font-pixel-size')
    property int horizontal_padding: Theme.getValue('button-padding-horizontal')
    property int vertical_padding: Theme.getValue('button-padding-vertical')

    width: childrenRect.width + padding + horizontal_padding
    height: childrenRect.height + padding + vertical_padding


    Rectangle{
        id: background
        width: parent.width
        radius: parent.radius
        height: parent.height

        color:
            mousearea.pressed ?
                parent.background_color_pressed
            :   mousearea.containsMouse ?
                    parent.background_color_hover
                :   parent.background_color

        MouseArea{
            id: mousearea
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: this.containsMouse ? Qt.PointingHandCursor : Qt.ArrowCursor
        }
    }

    Label{
        color: parent.color
        anchors.centerIn: parent
        text: parent.text
        font.pixelSize: parent.font_size
    }
}
