# 配置rust开发环境
## 首先确认是否已经安装了`rust`开发环境
运行命令
```bash
rustup default
```
如果返回
```
nightly-x86_64-unknown-linux-gnu (default)
```
或者
```
stable-x86_64-unknown-linux-gnu (default)
```
则说明`rustup`环境已经安装并且完成配置，去往[下载本控制程序代码](#下载本控制程序代码)

如果返回
```
error: no default toolchain configured
```
则说明已经安装了`rustup`，但是尚未进行配置，去往[配置toolchain channel 为stable](#配置toolchain-channel为stable)

如果返回找不到命令(command not found)，则说明`rustup`未安装，去往[安装rustup](#安装rustup)。

## 安装`rustup`
有两种方法：
1. 运行如下命令安装rustup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. 在`ubuntu`发行版上，可利用`snap`包管理器安装
```bash
sudo snap install rustup
```


## 配置toolchain channel为stable
不论使用上述哪种方法安装了`rustup`都需要用如下命令进行配置。
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
