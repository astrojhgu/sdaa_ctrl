#import "simplepaper.typ": *

#show figure: set block(breakable: true)

#import "utils.typ":new_content

#show: project.with(
  title: [SDANDART控制协议], authors: (
  ), keywords: (), date: [2024-12-26 version 0.9], abstract: [
    
    版本历史：
    - 20241106 v0.1 初始版本
    - 20241116 v0.2 I2C读写指令增加一个写入或者读回的字节数字段
    - 20241116 v0.3 状态查询的上行回复消息中，增加了健康指标个数的字段
    - 20241123 v0.4 增加VGA控制指令
    - 20241204 v0.5 状态查询的上行消息中，增加表示数据是否在传输的字段
    - 20241204 v0.6 增加休眠指令，对于状态查询的上行消息做进一步细化
    - 20241224 v0.7 在状态查询的上行消息健康指标之前增加一个magic number字段
    - 20241224 v0.8 加入一条对系统进行初始化的指令
    - 20241226 v0.9 扩展休眠控制指令，利用原先的保留位控制是进入休眠还是从休眠中唤醒，将非法指令重新解释为指令错误，包括对未知指令的回复和对操作失败的回复
  ],
)

#include "general_def.typ"

#include "query.typ"

#include "sync.typ"

#include "xgbe.typ"

#include "i2c.typ"

#include "stream_ctrl.typ"

//#include "vga.typ"

#include "suspend.typ"

#include "init.typ"

//#bibliography("ref.bib")
