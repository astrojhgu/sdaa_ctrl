#import "simplepaper.typ": *

#show figure: set block(breakable: true)

#import "utils.typ":new_content

#show: project.with(
  title: [SDAA控制协议], authors: (
  ), keywords: (), date: [2024-12-26], abstract: [
    
    版本历史：
    - 20250403 初始版本
  ],
)

#include "general_def.typ"

#include "query.typ"

#include "sync.typ"

#include "xgbe_var.typ"

#include "i2c.typ"

#include "stream_ctrl.typ"

//#include "vga.typ"

//#include "suspend.typ"

#include "init.typ"

//#bibliography("ref.bib")
