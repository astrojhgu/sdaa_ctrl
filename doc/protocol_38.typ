#import "simplepaper.typ": *

#show figure: set block(breakable: true)

#import "utils.typ":new_content

#show: project.with(
  title: [SDAA控制协议], authors: (
  ), keywords: (), date: [2024-12-26], abstract: [
    
    版本历史：
    - 20241106 初始版本
    - 20241116 状态查询的上行回复消息中，增加了健康指标个数的字段
    - 20241204 状态查询的上行消息中，增加表示数据是否在传输的字段
    - 20241224 在状态查询的上行消息健康指标之前增加一个magic number字段
  ],
)

#include "general_def.typ"

#include "query.typ"

#include "sync.typ"

#include "xgbe.typ"

//#include "i2c.typ"

#include "stream_ctrl.typ"

//#include "vga.typ"

//#include "suspend.typ"

//#include "init.typ"

//#bibliography("ref.bib")
