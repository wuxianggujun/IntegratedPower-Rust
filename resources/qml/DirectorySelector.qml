import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Dialogs

// 目录选择组件
Item {
    id: root

    property string inputDirectory: ""
    property string outputDirectory: ""
    property int fileCount: 0

    signal inputDirectoryChanged(string path)
    signal outputDirectoryChanged(string path)

    ColumnLayout {
        anchors.fill: parent
        spacing: 15

        // 输入目录选择
        GroupBox {
            Layout.fillWidth: true
            title: "输入目录"

            ColumnLayout {
                anchors.fill: parent
                spacing: 10

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Label {
                        text: "📁"
                        font.pixelSize: 24
                    }

                    TextField {
                        id: inputDirField
                        Layout.fillWidth: true
                        text: root.inputDirectory
                        placeholderText: "选择包含 Excel 文件的输入目录..."
                        readOnly: true

                        background: Rectangle {
                            color: inputDirField.text ? "#E8F5E9" : "#FFF"
                            border.color: inputDirField.text ? "#4CAF50" : "#E0E0E0"
                            border.width: 1
                            radius: 4
                        }
                    }

                    Button {
                        text: "浏览..."
                        onClicked: inputFolderDialog.open()

                        background: Rectangle {
                            color: parent.pressed ? "#1976D2" : (parent.hovered ? "#1E88E5" : "#2196F3")
                            radius: 4
                        }

                        contentItem: Text {
                            text: parent.text
                            color: "white"
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }

                    Button {
                        text: "✕"
                        visible: root.inputDirectory !== ""
                        onClicked: {
                            root.inputDirectory = ""
                            root.fileCount = 0
                            inputDirectoryChanged("")
                        }

                        background: Rectangle {
                            color: parent.pressed ? "#C62828" : (parent.hovered ? "#D32F2F" : "#F44336")
                            radius: 4
                        }

                        contentItem: Text {
                            text: parent.text
                            color: "white"
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }
                }

                // 文件统计
                Label {
                    visible: root.inputDirectory !== ""
                    text: "找到 " + root.fileCount + " 个 Excel 文件"
                    color: root.fileCount > 0 ? "#4CAF50" : "#FF9800"
                    font.pixelSize: 12
                }
            }
        }

        // 输出目录选择
        GroupBox {
            Layout.fillWidth: true
            title: "输出目录"

            ColumnLayout {
                anchors.fill: parent
                spacing: 10

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Label {
                        text: "📁"
                        font.pixelSize: 24
                    }

                    TextField {
                        id: outputDirField
                        Layout.fillWidth: true
                        text: root.outputDirectory
                        placeholderText: "选择保存处理结果的输出目录..."
                        readOnly: true

                        background: Rectangle {
                            color: outputDirField.text ? "#E3F2FD" : "#FFF"
                            border.color: outputDirField.text ? "#2196F3" : "#E0E0E0"
                            border.width: 1
                            radius: 4
                        }
                    }

                    Button {
                        text: "浏览..."
                        onClicked: outputFolderDialog.open()

                        background: Rectangle {
                            color: parent.pressed ? "#1976D2" : (parent.hovered ? "#1E88E5" : "#2196F3")
                            radius: 4
                        }

                        contentItem: Text {
                            text: parent.text
                            color: "white"
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }

                    Button {
                        text: "✕"
                        visible: root.outputDirectory !== ""
                        onClicked: {
                            root.outputDirectory = ""
                            outputDirectoryChanged("")
                        }

                        background: Rectangle {
                            color: parent.pressed ? "#C62828" : (parent.hovered ? "#D32F2F" : "#F44336")
                            radius: 4
                        }

                        contentItem: Text {
                            text: parent.text
                            color: "white"
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }
                }

                // 提示信息
                Label {
                    visible: root.outputDirectory !== ""
                    text: "✓ 输出目录已设置"
                    color: "#2196F3"
                    font.pixelSize: 12
                }
            }
        }

        // 验证状态
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 60
            color: root.inputDirectory && root.outputDirectory ? "#E8F5E9" : "#FFF3E0"
            border.color: root.inputDirectory && root.outputDirectory ? "#4CAF50" : "#FF9800"
            border.width: 1
            radius: 4
            visible: root.inputDirectory || root.outputDirectory

            RowLayout {
                anchors.fill: parent
                anchors.margins: 15
                spacing: 10

                Label {
                    text: root.inputDirectory && root.outputDirectory ? "✓" : "⚠"
                    font.pixelSize: 24
                    color: root.inputDirectory && root.outputDirectory ? "#4CAF50" : "#FF9800"
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2

                    Label {
                        text: root.inputDirectory && root.outputDirectory ? "准备就绪" : "请完成目录选择"
                        font.bold: true
                        color: root.inputDirectory && root.outputDirectory ? "#4CAF50" : "#FF9800"
                    }

                    Label {
                        text: root.inputDirectory && root.outputDirectory 
                            ? "所有必需的目录已设置，可以开始处理" 
                            : "请选择输入和输出目录"
                        font.pixelSize: 12
                        color: "#757575"
                    }
                }
            }
        }
    }

    // 输入目录选择对话框
    FolderDialog {
        id: inputFolderDialog
        title: "选择输入目录"
        onAccepted: {
            root.inputDirectory = folder.toString().replace("file:///", "")
            inputDirectoryChanged(root.inputDirectory)
            // 这里应该调用 Rust 后端来统计文件数量
            // root.fileCount = backend.countFiles(root.inputDirectory)
        }
    }

    // 输出目录选择对话框
    FolderDialog {
        id: outputFolderDialog
        title: "选择输出目录"
        onAccepted: {
            root.outputDirectory = folder.toString().replace("file:///", "")
            outputDirectoryChanged(root.outputDirectory)
        }
    }
}
