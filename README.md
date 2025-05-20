### Приложение для отправки мусора на адрес и порт сервера WireGuard перед подключением.

Пример файла конфигурации wireguard клиента
```
[Interface]
PrivateKey = {{Private Key}}
ListenPort = local_listen_port
Address = 10.5.0.2/32
DNS = 1.1.1.1
PreUp = C:\usr\wgdpi.exe -s endpoint_ip:endpoint_port -l local_listen_port

[Peer]
PublicKey = {{Public Key}}
AllowedIPs = 0.0.0.0/0
Endpoint = endpoint_ip:endpoint_port
PersistentKeepalive = 15
```


Для того, чтобы работала опция `PreUp`, нужно внести изменения в реестр.
```
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SOFTWARE\WireGuard]
"DangerousScriptExecution"=dword:00000001
```
