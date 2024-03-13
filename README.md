# Switch Command Conversion Tool
[中文](./README-CN.md)
## At the beginning, this project is only a demo and is extremely incomplete
The Switch Command Conversion Tool is a simple command-line utility designed to convert configuration commands from Huawei switches to commands compatible with H3C switches.
Features

    Parses configuration files from Huawei switches and converts them into configuration files compatible with H3C switches.
    Supports conversion of commands including interface configuration, IP address configuration, and VLAN configuration.

Usage

    Save the configuration file from your Huawei switch as a .txt file.
    Run the tool from the command line, specifying the path to the input file and the desired output file.


```bash
./huawei -i input.txt -o output.txt
```
    The tool will read the configuration from the input file and generate the corresponding configuration file for H3C switches.

Notes

    This tool currently supports basic conversion and may not cover all scenarios.
    It's recommended to backup your original configuration file before using the tool, to prevent accidental data loss.