#import "utils.typ":new_content

= VGA控制
== 下行指令
#figure(caption:"VGA增益控制")[
  #table(columns: (auto,auto,auto),
  table.header([偏移],[内容],[解释]),
  [0:3],[06 00 00 00],[消息类型],
  [4:7],[?? ?? ?? ??],[可定制的消息序列号],
  [8:11], [?? ?? ?? ??], [VGA通道个数],
  [12:15], [?? ?? ?? ??], [VGA增益值],
  [...],[...],[...]
  )
]

== 上行消息
#figure(caption:"VGA增益控制")[
  #table(columns: (auto,auto,auto),
  table.header([偏移],[内容],[解释]),
  [0:3],[06 00 00 ff],[消息类型],
  [4:7],[?? ?? ?? ??],[可定制的消息序列号],
  [8:11],[?? ?? ?? ??],[返回错误码，全零代表成果],  
  )
]

