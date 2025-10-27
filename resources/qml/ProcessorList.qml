import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

// 处理器列表组件
Item {
    id: root

    property var processors: []
    property string selectedProcessorId: ""
    property string searchQuery: ""

    signal processorSelected(string processorId)
    signal processorDoubleClicked(string processorId)

    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        // 搜索栏
        TextField {
            id: searchField
            Layout.fillWidth: true
            placeholderText: "🔍 搜索处理器..."
            text: root.searchQuery
            onTextChanged: root.searchQuery = text

            background: Rectangle {
                color: "white"
                border.color: searchField.activeFocus ? "#2196F3" : "#E0E0E0"
                border.width: searchField.activeFocus ? 2 : 1
                radius: 4
            }
        }

        // 处理器列表
        ListView {
            id: processorListView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 8

            model: ListModel {
                id: processorModel
            }

            delegate: Rectangle {
                width: processorListView.width
                height: 100
                color: model.selected ? "#2196F3" : (mouseArea.containsMouse ? "#F5F5F5" : "white")
                radius: 8
                border.color: model.selected ? "#2196F3" : "#E0E0E0"
                border.width: model.selected ? 2 : 1

                // 不可用时的遮罩
                Rectangle {
                    anchors.fill: parent
                    color: "#80FFFFFF"
                    radius: 8
                    visible: !model.available

                    Label {
                        anchors.centerIn: parent
                        text: "不可用"
                        color: "#757575"
                        font.bold: true
                    }
                }

                MouseArea {
                    id: mouseArea
                    anchors.fill: parent
                    hoverEnabled: true
                    enabled: model.available

                    onClicked: {
                        root.selectedProcessorId = model.processorId
                        processorSelected(model.processorId)
                    }

                    onDoubleClicked: {
                        processorDoubleClicked(model.processorId)
                    }
                }

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 12

                    // 图标
                    Rectangle {
                        Layout.preferredWidth: 60
                        Layout.preferredHeight: 60
                        color: model.selected ? "white" : "#F5F5F5"
                        radius: 8

                        Label {
                            anchors.centerIn: parent
                            text: model.icon
                            font.pixelSize: 36
                        }
                    }

                    // 信息
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: 4

                        // 名称和版本
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Label {
                                text: model.name
                                font.pixelSize: 16
                                font.bold: true
                                color: model.selected ? "white" : "#212121"
                                Layout.fillWidth: true
                            }

                            Label {
                                text: "v" + model.version
                                font.pixelSize: 11
                                color: model.selected ? "white" : "#757575"
                            }
                        }

                        // 描述
                        Label {
                            text: model.description
                            font.pixelSize: 13
                            color: model.selected ? "white" : "#757575"
                            wrapMode: Text.WordWrap
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                        }

                        // ID（小字）
                        Label {
                            text: "ID: " + model.processorId
                            font.pixelSize: 10
                            color: model.selected ? "white" : "#BDBDBD"
                            font.family: "Consolas, monospace"
                        }
                    }

                    // 选中指示器
                    Rectangle {
                        Layout.preferredWidth: 4
                        Layout.fillHeight: true
                        color: "#4CAF50"
                        radius: 2
                        visible: model.selected
                    }
                }
            }

            // 空状态
            Label {
                anchors.centerIn: parent
                text: root.searchQuery ? "未找到匹配的处理器" : "没有可用的处理器"
                color: "#757575"
                font.pixelSize: 14
                visible: processorListView.count === 0
            }

            ScrollBar.vertical: ScrollBar {
                policy: ScrollBar.AsNeeded
            }
        }

        // 统计信息
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 40
            color: "#F5F5F5"
            radius: 4

            RowLayout {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 15

                Label {
                    text: "📊 总计: " + processorListView.count
                    font.pixelSize: 12
                    color: "#757575"
                }

                Rectangle {
                    width: 1
                    height: 20
                    color: "#E0E0E0"
                }

                Label {
                    text: "✓ 可用: " + getAvailableCount()
                    font.pixelSize: 12
                    color: "#4CAF50"
                }

                Item {
                    Layout.fillWidth: true
                }

                Label {
                    text: root.selectedProcessorId ? "已选择" : "未选择"
                    font.pixelSize: 12
                    color: root.selectedProcessorId ? "#2196F3" : "#757575"
                    font.bold: root.selectedProcessorId
                }
            }
        }
    }

    // 辅助函数
    function getAvailableCount() {
        var count = 0
        for (var i = 0; i < processorModel.count; i++) {
            if (processorModel.get(i).available) {
                count++
            }
        }
        return count
    }

    // 更新处理器列表
    function updateProcessors(processorList) {
        processorModel.clear()
        for (var i = 0; i < processorList.length; i++) {
            var proc = processorList[i]
            processorModel.append({
                processorId: proc.id,
                name: proc.name,
                description: proc.description,
                icon: proc.icon || "📊",
                version: proc.version,
                available: proc.available,
                selected: proc.id === root.selectedProcessorId
            })
        }
    }

    // 选择处理器
    function selectProcessor(processorId) {
        root.selectedProcessorId = processorId
        for (var i = 0; i < processorModel.count; i++) {
            processorModel.setProperty(i, "selected", processorModel.get(i).processorId === processorId)
        }
    }

    // 清除选择
    function clearSelection() {
        root.selectedProcessorId = ""
        for (var i = 0; i < processorModel.count; i++) {
            processorModel.setProperty(i, "selected", false)
        }
    }

    // 过滤处理器
    function filterProcessors(query) {
        root.searchQuery = query
        // 这里应该调用 Rust 后端进行过滤
        // 简化实现：在 QML 中过滤
    }
}
