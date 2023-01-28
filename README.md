# 插件示例

[A Plugin System in Rust](https://nullderef.com/series/rust-plugins/)

## abi_stable

执行make build, 输出

```shell

load "plugin_fw" success
load "plugin_server" success

command:
{
			"header": {
				"request_id": "{{device_id}}-13",
				"msg_type": "request",
				"version": "1.0",
				"created": 1539355895215,
				"sender": "gateway"
			},
			"command": {
				"action": "set",
				"target": {
					"artifact": {
						"mime_type": "cmd",
						"payload": {
							"data": [
								[
									"show arp dynamic"
								],
								[
									"show arp dynamic"
								]
							]
						}
					}
				},
				"actuator": {
					"actuator_type": "device",
					"actuator_id": [
						"{{device_id}}"
					]
				},
				"args": {
					"start_time": 1534775460000,
					"stop_time": 1934775460000,
					"response_requested": "Complete"
				}
			}
		}
reponse:
    send messge to plugin firewall success
from:
    "plugin_fw"


command:
{
			"header": {
				"request_id": "{{device_id}}-13",
				"msg_type": "request",
				"version": "1.0",
				"created": 1539355895215,
				"sender": "gateway"
			},
			"command": {
				"action": "set",
				"target": {
					"artifact": {
						"mime_type": "cmd",
						"payload": {
							"data": [
								[
									"ip a"
								]
							]
						}
					}
				},
				"actuator": {
					"actuator_type": "device",
					"actuator_id": [
						"{{device_id}}"
					]
				},
				"args": {
					"start_time": 1534775460000,
					"stop_time": 1934775460000,
					"response_requested": "Complete"
				}
			}
		}
reponse:
    send messge to plugin server success
from:
    "plugin_server"
```