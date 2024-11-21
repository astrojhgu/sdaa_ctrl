# 配置rust开发环境
1. 运行命令安装rustup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 配置toolchain channel为stable
```bash
rustup default stable
```

完成

# 下载本控制程序代码
```bash
git clone https://github.com/astrojhgu/sdand_ctrl.git
```

# 编译
```bash
cd sdand_ctrl
cargo build --release
```

# 使用
## 发送控制指令
控制指令数据模板位于目录[`cmd/`](cmd)，可打开修改

发送指令，假定两台设备的ip地址分别是192.168.1.100和192.168.1.101，并且开放端口号3000用以接收指令。本机开启3001号端口用于接收反馈消息。

这里发送[cmd/Query.yaml](cmd/Query.yaml)指令
```bash
cargo run --bin send_cmd --release -- --addr 192.168.1.100:3000 192.168.1.101:3000 -L '[::]:3001' -c cmd/Query.yaml
```

## 启动虚拟设备服务器，用以调试指令发送程序
```bash
cargo run --bin dummy_server --release -- --addr '[::]:3000' 
```
