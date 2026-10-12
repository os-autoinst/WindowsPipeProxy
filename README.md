# Windows Pipe Proxy

The Windows Pipe Proxy or WPP is a named pipe to tcp proxy. It acts as a bridge between a set of [Windows named pipes](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipes) and TCP sockets.
Each named pipe will be exposed to a defined TCP socket. The tool can be used for connecting a serial port on a Hyper-V hypervisor to an [openQA](https://open.qa/) instance.

Additional documentation is provided in [docs/](docs/).

## Usage

WPP acts as a drop-in replacement for the aging `Named Pipe TCP Proxy`. It is configured via a [configuration file](pipe-proxy.toml).
The program supports a set of named pipes and will run just in the background. Once a named pipe appears, it will open a single TCP socket on a defined local address for this pipe.

To connect to a Hyper-V instance, one needs to first add a COM to named pipe option to the virtual machine, e.g. for a VM named `jellyfish` one could run:

```
Set-VMComPort -VMName jellyfish -Number 1 -Path \\.\pipe\jellyfish
```

Then run the program and pass the configuration file as argument:

```powershell
.\windows-pipe-proxy.exe pipe-proxy.toml
```

### Register as a Windows service

By default, the service expects the configuration file to be present in `C:\pipe-proxy\config.toml`.
You can change this by adding a configuration file argument.

To define a Service with the default parameters, use `sc.exe` (including the spaces after `binPath=` and `start=`)

```
sc.exe create "PipeProxy" binPath= "C:\pipe-proxy\windows-pipe-proxy.exe" start= auto
sc.exe start "PipeProxy"
```

To delete the said service again:

```
sc.exe delete "PipeProxy"
```

## Credits

* Inspired by https://github.com/pratikpc/named-pipe-to-tcp-proxy
