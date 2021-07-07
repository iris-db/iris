# Iris Build System

The Iris Build Sytstem (IBS) is a build system that easily builds the project and runs tests accross multiple modules.

## Usage

Start by building the script.

```shell
go build -o ibs ibs.go
```

Run the IBS script. Specify whether the script to be run is a test or build script and provide the name with the CLI 
flags.

```shell
./ibs --type Build --name Iris-Server
```
