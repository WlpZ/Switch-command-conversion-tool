# Switch Command Conversion Tool
[English](./README.md)
### !!!写在开头 这个项目仅仅是一个demo，及其不完善
该工具是一个简单的命令转换工具，用于将华为交换机配置命令转换为新华三交换机配置命令。
功能

    支持解析华为交换机配置文件，并将其转换为新华三交换机配置文件。
    支持转换的命令包括接口配置、IP 地址配置和 VLAN 配置等。

使用方法

    将华为交换机的配置文件保存为 .txt 格式。
    在命令行下运行该工具，并指定输入文件的路径和输出文件的路径。



```    ./huawei -i input.txt -o output.txt```

    工具将会读取输入文件中的华为交换机配置，并生成相应的新华三交换机配置文件。

注意事项

    该工具目前仅支持简单的配置转换，可能无法覆盖所有情况。
    在使用工具之前，请备份原始配置文件，以防止意外数据丢失。