import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    id: mainWindow
    visible: true
    width: 1200
    height: 800
    title: "IntegratedPower"

    // 主布局
    RowLayout {
        anchors.fill: parent
        spacing: 0

        // 左侧边栏
        Rectangle {
            Layout.preferredWidth: 250
            Layout.fillHeight: true
            color: "#F5F5F5"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 10

                // 搜索框
                TextField {
                    id: searchField
                    Layout.fillWidth: true
                    placeholderText: "搜索功能..."
                    
                    background: Rectangle {
                        color: "white"
                        border.color: "#E0E0E0"
                        border.width: 1
                        radius: 4
                    }
                }

                // 功能列表
                ListView {
                    id: processorListView
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    spacing: 5

                    model: ListModel {
                        ListElement {
                            processorId: "data_cleaning"
                            processorName: "数据清洗"
                            processorIcon: "🧹"
                            processorDescription: "删除空行、重复行"
                        }
                        ListElement {
                            processorId: "data_statistics"
                            processorName: "数据统计"
                            processorIcon: "📊"
                            processorDescription: "添加行号，计算统计"
                        }
                    }

                    delegate: Rectangle {
                        width: processorListView.width
                        height: 80
                        color: processorListView.currentIndex === index ? "#2196F3" : "white"
                        radius: 4
                        border.color: "#E0E0E0"
                        border.width: 1

                        MouseArea {
                            anchors.fill: parent
                            onClicked: processorListView.currentIndex = index
                        }

                        RowLayout {
                            anchors.fill: parent
                            anchors.margins: 10
                            spacing: 10

                            Text {
                                text: processorIcon
                                font.pixelSize: 32
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 2

                                Text {
                                    text: processorName
                                    font.pixelSize: 16
                                    font.bold: true
                                    color: processorListView.currentIndex === index ? "white" : "#212121"
                                }

                                Text {
                                    text: processorDescription
                                    font.pixelSize: 12
                                    color: processorListView.currentIndex === index ? "white" : "#757575"
                                    wrapMode: Text.WordWrap
                                    Layout.fillWidth: true
                                }
                            }
                        }
                    }
                }
            }
        }

        // 主工作区
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "white"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 20
                spacing: 20

                // 顶部工具栏
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Button {
                        text: "🏠 主页"
                        flat: true
                    }

                    Button {
                        text: "⚙️ 设置"
                        flat: true
                    }

                    Button {
                        text: "📜 历史"
                        flat: true
                    }

                    Item {
                        Layout.fillWidth: true
                    }

                    Button {
                        text: "🌙 主题"
                        flat: true
                    }
                }

                // 功能选择区域
                GroupBox {
                    Layout.fillWidth: true
                    title: "选择处理功能"

                    GridLayout {
                        anchors.fill: parent
                        columns: 3
                        rowSpacing: 10
                        columnSpacing: 10

                        Repeater {
                            model: 6
                            
                            Rectangle {
                                Layout.preferredWidth: 150
                                Layout.preferredHeight: 120
                                color: "#F5F5F5"
                                radius: 8
                                border.color: "#E0E0E0"
                                border.width: 1

                                ColumnLayout {
                                    anchors.centerIn: parent
                                    spacing: 10

                                    Text {
                                        text: "📊"
                                        font.pixelSize: 48
                                        Layout.alignment: Qt.AlignHCenter
                                    }

                                    Text {
                                        text: "功能 " + (index + 1)
                                        font.pixelSize: 14
                                        Layout.alignment: Qt.AlignHCenter
                                    }
                                }

                                MouseArea {
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    onEntered: parent.color = "#E0E0E0"
                                    onExited: parent.color = "#F5F5F5"
                                    onClicked: console.log("Clicked function " + (index + 1))
                                }
                            }
                        }
                    }
                }

                // 目录选择区域
                GroupBox {
                    Layout.fillWidth: true
                    title: "目录设置"

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: 10

                        // 输入目录
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 10

                            Label {
                                text: "📁 输入目录:"
                                Layout.preferredWidth: 100
                            }

                            TextField {
                                id: inputDirField
                                Layout.fillWidth: true
                                placeholderText: "选择输入目录..."
                                readOnly: true
                            }

                            Button {
                                text: "浏览..."
                                onClicked: {
                                    // 打开文件对话框
                                }
                            }
                        }

                        // 输出目录
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 10

                            Label {
                                text: "📁 输出目录:"
                                Layout.preferredWidth: 100
                            }

                            TextField {
                                id: outputDirField
                                Layout.fillWidth: true
                                placeholderText: "选择输出目录..."
                                readOnly: true
                            }

                            Button {
                                text: "浏览..."
                                onClicked: {
                                    // 打开文件对话框
                                }
                            }
                        }
                    }
                }

                // 开始处理按钮
                Button {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 50
                    text: "开始处理"
                    
                    background: Rectangle {
                        color: parent.pressed ? "#1976D2" : (parent.hovered ? "#1E88E5" : "#2196F3")
                        radius: 4
                    }

                    contentItem: Text {
                        text: parent.text
                        font.pixelSize: 16
                        font.bold: true
                        color: "white"
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                // 进度显示区域
                GroupBox {
                    id: progressGroup
                    Layout.fillWidth: true
                    title: "处理进度"
                    visible: false

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: 10

                        Label {
                            id: currentFileLabel
                            text: "当前文件: data_2024_01.xlsx"
                        }

                        ProgressBar {
                            id: progressBar
                            Layout.fillWidth: true
                            value: 0.65
                        }

                        Label {
                            id: progressStatsLabel
                            text: "已处理: 13/20 文件 | 成功: 12 | 失败: 1"
                        }

                        Button {
                            Layout.alignment: Qt.AlignHCenter
                            text: "取消处理"
                            onClicked: {
                                // 取消处理
                            }
                        }
                    }
                }

                Item {
                    Layout.fillHeight: true
                }
            }
        }
    }

    // 状态栏
    footer: ToolBar {
        RowLayout {
            anchors.fill: parent
            anchors.margins: 5

            Label {
                text: "就绪"
                Layout.fillWidth: true
            }

            Label {
                text: "0 个文件待处理"
            }
        }
    }
}
