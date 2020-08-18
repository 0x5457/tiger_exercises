# 习题
## 2.1 写出下面每一种单词的正则表达式。
### a. 字母表 { a, b, c }上满足后面条件的字符串: 首次出现的 a 位于首次出现的 b 之前。
  
  `[ac]*[bc]*`

### b. 字母表 { a, b, c } 上由偶数个 a 组成的字符串。
 
  `[bc]*(a[bc]*a)*`

### c. 是 4 的倍数的二进制数。

  `[10]+00`

### d. 大于 101001 的二进制数。
  
  `[1[10]{6,}|11[10]{4}|1010*1{1,2}0*[01]]]`

### e. 字母表 { a, b, c } 上不包含连续子串 baa 的字符串。
 
  `[ac]*b*a?[bc]*`

### f. C 语言中非负整常数组成的语言，其中以 0 开头

  `[0[0-7]*|[1-9]\d*]`


### g. 使得方程![a^n+b^n=c^n](https://render.githubusercontent.com/render/math?math=a%5En%2Bb%5En%3Dc%5En)存在着整数解的二进制整数n。

  `1|10`


## 2.2 对于下列描述，试解释为什么不存在对应的正则表达式。

### a. 由 a 和 b  组成的字符串，其中 a 的个数要多于 b。
`答案:` ***因为正则表达式没有记忆能力***
### b. 由 a 和 b 组成的会问字符串（顺序于倒数相同）。
`答案`: ***因为正则表达式没有记忆能力***
### c. 语法上正确的 C 程序。
`答案:` ***因为正则表达式无法表达递归嵌套结构***

## 2.3 用自然语言描述下述有限状态自动机识别的语言。
### a.
<!-- This is the original graph
digraph G {

   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];1;2;3;4;6;7;8;9}
   {node[shape=doublecircle];10}
   
   start->1
   1->2[label="0"]
   1->6[label="1"]
   2->3[label="1"]
   2->7[label="0"]
   3->4[label="1"]
   3->8[label="0"]
   4->9[label="0"]
   4->10[label="1"]
   6->7[label="0"]
   6->7[label="1"]
   7->8[label="0"]
   7->8[label="1"]
   8->10[label="0"]
   8->10[label="1"]
   
   rankdir="LR"
}
-->
![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B4%3B6%3B7%3B8%3B9%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B10%7D%0A%20%20%20%0A%20%20%20start-%3E1%0A%20%20%201-%3E2%5Blabel%3D%220%22%5D%0A%20%20%201-%3E6%5Blabel%3D%221%22%5D%0A%20%20%202-%3E3%5Blabel%3D%221%22%5D%0A%20%20%202-%3E7%5Blabel%3D%220%22%5D%0A%20%20%203-%3E4%5Blabel%3D%221%22%5D%0A%20%20%203-%3E8%5Blabel%3D%220%22%5D%0A%20%20%204-%3E9%5Blabel%3D%220%22%5D%0A%20%20%204-%3E10%5Blabel%3D%221%22%5D%0A%20%20%206-%3E7%5Blabel%3D%220%22%5D%0A%20%20%206-%3E7%5Blabel%3D%221%22%5D%0A%20%20%207-%3E8%5Blabel%3D%220%22%5D%0A%20%20%207-%3E8%5Blabel%3D%221%22%5D%0A%20%20%208-%3E10%5Blabel%3D%220%22%5D%0A%20%20%208-%3E10%5Blabel%3D%221%22%5D%0A%20%20%20%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

`不能为 0110 的 4 位二进制字符串`

### b.
<!-- This is the original graph
digraph G {

   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];" ";"   ";"    ";"     ";}
   {node[shape=doublecircle];"  "}
  
   start->" "
   " "->"  "[label="a"]
   "  "->"   "[label="a"]
   "   "->"    "[label="a"]
   "    "->"     "[label="a"]
   "     "->" "[label="a"]
   
   rankdir="LR";
}
-->
![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B%22%20%22%3B%22%20%20%20%22%3B%22%20%20%20%20%22%3B%22%20%20%20%20%20%22%3B%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B%22%20%20%22%7D%0A%20%20%0A%20%20%20start-%3E%22%20%22%0A%20%20%20%22%20%22-%3E%22%20%20%22%5Blabel%3D%22a%22%5D%0A%20%20%20%22%20%20%22-%3E%22%20%20%20%22%5Blabel%3D%22a%22%5D%0A%20%20%20%22%20%20%20%22-%3E%22%20%20%20%20%22%5Blabel%3D%22a%22%5D%0A%20%20%20%22%20%20%20%20%22-%3E%22%20%20%20%20%20%22%5Blabel%3D%22a%22%5D%0A%20%20%20%22%20%20%20%20%20%22-%3E%22%20%22%5Blabel%3D%22a%22%5D%0A%20%20%20%0A%20%20%20rankdir%3D%22LR%22%3B%0A%7D)

`由`![5 ^ n + 1](https://render.githubusercontent.com/render/math?math=5%5En%2B1) `个 a 组成的字符串`


### c. 
<!-- This is the original graph

digraph C {
   start[label= "", shape=none,height=.0,width=.0]
   
   {node[shape=circle];1;2}
   {node[shape=doublecircle];0}
   
   start->0
   0->0[label="0"]
   0->1[label="1"]
   1->0[label="1"]
   1->2[label="0"]
   2->1[label="0"]
   2->2[label="1"]
   rankdir="LR"
}
 -->
![c](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B0%7D%0A%20%20%20%0A%20%20%20start-%3E0%0A%20%20%200-%3E0%5Blabel%3D%220%22%5D%0A%20%20%200-%3E1%5Blabel%3D%221%22%5D%0A%20%20%201-%3E0%5Blabel%3D%221%22%5D%0A%20%20%201-%3E2%5Blabel%3D%220%22%5D%0A%20%20%202-%3E1%5Blabel%3D%220%22%5D%0A%20%20%202-%3E2%5Blabel%3D%221%22%5D%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

`略`

## 2.4 将下面两个正则表达式转换为非确定的有限自动机。

<!-- This is the original graph
digraph A {
   start[label= "", shape=none,height=.0,width=.0]
   
   {node[shape=circle];0;1;3;4;5;7;8;9}
   {node[shape=doublecircle];2;6;10}
   
   start->0
   0->1[label="i"]
   1->2[label="f"]
   0->3[label="t"]
   3->4[label="h"]
   4->5[label="e"]
   5->6[label="n"]
   0->7[label="e"]
   7->8[label="l"]
   8->9[label="s"]
   9->10[label="e"]
   rankdir="LR"
}
 -->
### a. (if|then|else)

![a](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B0%3B1%3B3%3B4%3B5%3B7%3B8%3B9%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B2%3B6%3B10%7D%0A%20%20%20%0A%20%20%20start-%3E0%0A%20%20%200-%3E1%5Blabel%3D%22i%22%5D%0A%20%20%201-%3E2%5Blabel%3D%22f%22%5D%0A%20%20%200-%3E3%5Blabel%3D%22t%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22h%22%5D%0A%20%20%204-%3E5%5Blabel%3D%22e%22%5D%0A%20%20%205-%3E6%5Blabel%3D%22n%22%5D%0A%20%20%200-%3E7%5Blabel%3D%22e%22%5D%0A%20%20%207-%3E8%5Blabel%3D%22l%22%5D%0A%20%20%208-%3E9%5Blabel%3D%22s%22%5D%0A%20%20%209-%3E10%5Blabel%3D%22e%22%5D%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

### b. a((b|a*c)x)*|x*a

<!-- This is the original graph
digraph B {
   start[label= "", shape=none,height=.0,width=.0];
   {node[shape=circle];1;2;3;4;7};
   {node[shape=doublecircle];6;8}
   
   start->1
   1->2[label="a"]
   2->4[label="b"]
   2->3[label="ε"]
   3->3[label="a"]
   3->4[label="c"]
   4->6[label="x"]
   1->7[label="ε"]
   7->7[label="x"]
   7->8[label="a"]
   2->6[label="ε"]
   6->2[label="ε"]
   
   rankdir="LR";
}
-->
![b](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%3B%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B4%3B7%7D%3B%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B6%3B8%7D%0A%20%20%20%0A%20%20%20start-%3E1%0A%20%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E4%5Blabel%3D%22b%22%5D%0A%20%20%202-%3E3%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%203-%3E3%5Blabel%3D%22a%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22c%22%5D%0A%20%20%204-%3E6%5Blabel%3D%22x%22%5D%0A%20%20%201-%3E7%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%207-%3E7%5Blabel%3D%22x%22%5D%0A%20%20%207-%3E8%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E6%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%206-%3E2%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%20%0A%20%20%20rankdir%3D%22LR%22%3B%0A%7D)

## 2.5 将下面 NFA 转换为确定的有限自动机
### a.

<!-- This is the original graph
digraph G {

   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];1;2;3;4;5;6}
   {node[shape=doublecircle];7}
  
   start->1
   1->2[label="ε"]
   2->3[label="ε"]
   3->4[label="ε"]
   4->1[label="ε"]
   1->5[label="x"]
   5->2[label="z"]
   5->6[label="ε"]
   2->6[label="y"]
   6->7[label="ε"]
   
   rankdir="LR";
}
-->
![digraph](https://g.gravizo.com/svg?%0Adigraph%20G%20%7B%0A%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B4%3B5%3B6%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B7%7D%0A%20%20%0A%20%20%20start-%3E1%0A%20%20%201-%3E2%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%202-%3E3%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%204-%3E1%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%201-%3E5%5Blabel%3D%22x%22%5D%0A%20%20%205-%3E2%5Blabel%3D%22z%22%5D%0A%20%20%205-%3E6%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%202-%3E6%5Blabel%3D%22y%22%5D%0A%20%20%206-%3E7%5Blabel%3D%22%CE%B5%22%5D%0A%20%20%20%0A%20%20%20rankdir%3D%22LR%22%3B%0A%7D)

`答案：`
<!-- This is the original graph
digraph G {
   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];"1,2,3,4"}
   {node[shape=doublecircle];"5,6,7";"6,7"}
   start->"1,2,3,4"
   "1,2,3,4"->"5,6,7"[label="x"]
   "1,2,3,4"->"6,7"[label="y"]
   "5,6,7"->"1,2,3,4"[label="z"]
  
   rankdir="LR"
}
-->
![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B%221%2C2%2C3%2C4%22%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B%225%2C6%2C7%22%3B%226%2C7%22%7D%0A%20%20%20start-%3E%221%2C2%2C3%2C4%22%0A%20%20%20%221%2C2%2C3%2C4%22-%3E%225%2C6%2C7%22%5Blabel%3D%22x%22%5D%0A%20%20%20%221%2C2%2C3%2C4%22-%3E%226%2C7%22%5Blabel%3D%22y%22%5D%0A%20%20%20%225%2C6%2C7%22-%3E%221%2C2%2C3%2C4%22%5Blabel%3D%22z%22%5D%0A%20%20%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

### b.
<!-- This is the original graph
digraph B {
   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];1;2;3;4}
   {node[shape=doublecircle];6}
   start->1
   1->1[label="a"]
   1->1[label="b"]
   1->2[label="a"]
   2->3[label="a"]
   2->3[label="b"]
   3->4[label="a"]
   3->4[label="b"]
   4->5[label="a"]
   4->5[label="b"]
   5->6[label="a"]
   5->6[label="b"]
   rankdir="LR"
}
-->
![b](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B4%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B6%7D%0A%20%20%20start-%3E1%0A%20%20%201-%3E1%5Blabel%3D%22a%22%5D%0A%20%20%201-%3E1%5Blabel%3D%22b%22%5D%0A%20%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E3%5Blabel%3D%22a%22%5D%0A%20%20%202-%3E3%5Blabel%3D%22b%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22a%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22b%22%5D%0A%20%20%204-%3E5%5Blabel%3D%22a%22%5D%0A%20%20%204-%3E5%5Blabel%3D%22b%22%5D%0A%20%20%205-%3E6%5Blabel%3D%22a%22%5D%0A%20%20%205-%3E6%5Blabel%3D%22b%22%5D%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

`答案：`
<!-- This is the original graph
digraph B {

   start[label= "", shape=none,height=.0,width=.0]
   {node[shape=circle];1;"1,2";3;4;5}
   {node[shape=doublecircle];6}
  
   start->1
   1->"1,2"[label="a"]
   1->1[label="b"]
   "1,2"->"1,2"[label="a"]
   "1,2"->1[label="b"]
   "1,2"->3[label="a|b"]
   3->4[label="a|b"]
   4->5[label="a|b"]
   5->6[label="a|b"]
   rankdir="LR";
}
-->
![b](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%0A%20%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B%221%2C2%22%3B3%3B4%3B5%7D%0A%20%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B6%7D%0A%20%20%0A%20%20%20start-%3E1%0A%20%20%201-%3E%221%2C2%22%5Blabel%3D%22a%22%5D%0A%20%20%201-%3E1%5Blabel%3D%22b%22%5D%0A%20%20%20%221%2C2%22-%3E%221%2C2%22%5Blabel%3D%22a%22%5D%0A%20%20%20%221%2C2%22-%3E1%5Blabel%3D%22b%22%5D%0A%20%20%20%221%2C2%22-%3E3%5Blabel%3D%22a%7Cb%22%5D%0A%20%20%203-%3E4%5Blabel%3D%22a%7Cb%22%5D%0A%20%20%204-%3E5%5Blabel%3D%22a%7Cb%22%5D%0A%20%20%205-%3E6%5Blabel%3D%22a%7Cb%22%5D%0A%20%20%20rankdir%3D%22LR%22%3B%0A%7D)

### c.
<!-- This is the original graph
digraph C {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];1;2;3;5;6;7;8;10;11;12;14;15;16;17}
  {node[shape=doublecircle];4;9;13;18}
  
  start=1
  1->2[label="c"]
  2->3[label="a"]
  3->4[label="t"]
  1->5[label="ε"]
  5->6[label="c"]
  6->7[label="a"]
  7->8[label="t"]
  8->9[label="s"]
  5->10[label="ε"]
  10->11[label="c"]
  11->12[label="a"]
  12->13[label="r"]
  10->14[label="ε"]
  14->15[label="c"]
  15->16[label="a"]
  16->17[label="r"]
  17->18[label="s"]
  rankdir="LR"
}
-->
![c](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B5%3B6%3B7%3B8%3B10%3B11%3B12%3B14%3B15%3B16%3B17%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B4%3B9%3B13%3B18%7D%0A%20%20%0A%20%20start%3D1%0A%20%201-%3E2%5Blabel%3D%22c%22%5D%0A%20%202-%3E3%5Blabel%3D%22a%22%5D%0A%20%203-%3E4%5Blabel%3D%22t%22%5D%0A%20%201-%3E5%5Blabel%3D%22%CE%B5%22%5D%0A%20%205-%3E6%5Blabel%3D%22c%22%5D%0A%20%206-%3E7%5Blabel%3D%22a%22%5D%0A%20%207-%3E8%5Blabel%3D%22t%22%5D%0A%20%208-%3E9%5Blabel%3D%22s%22%5D%0A%20%205-%3E10%5Blabel%3D%22%CE%B5%22%5D%0A%20%2010-%3E11%5Blabel%3D%22c%22%5D%0A%20%2011-%3E12%5Blabel%3D%22a%22%5D%0A%20%2012-%3E13%5Blabel%3D%22r%22%5D%0A%20%2010-%3E14%5Blabel%3D%22%CE%B5%22%5D%0A%20%2014-%3E15%5Blabel%3D%22c%22%5D%0A%20%2015-%3E16%5Blabel%3D%22a%22%5D%0A%20%2016-%3E17%5Blabel%3D%22r%22%5D%0A%20%2017-%3E18%5Blabel%3D%22s%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)

`答案：`
<!-- This is the original graph
digraph C {

  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];"1,5,10,14";"2,6,11,15";"3,7,12,16"}
  {node[shape=doublecircle];"4,8";"13,17";9;18}
    
  start->"1,5,10,14"
  "1,5,10,14"->"2,6,11,15"[label="c"]
  "2,6,11,15"->"3,7,12,16"[label="a"]
  "3,7,12,16"->"4,8"[label="t"]
  "3,7,12,16"->"13,17"[label="r"]
  "4,8"->9[label="s"]
  "13,17"->18[label="s"]
  rankdir="LR"
}
-->

![c](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B%221%2C5%2C10%2C14%22%3B%222%2C6%2C11%2C15%22%3B%223%2C7%2C12%2C16%22%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B%224%2C8%22%3B%2213%2C17%22%3B9%3B18%7D%0A%20%20%20%20%0A%20%20start-%3E%221%2C5%2C10%2C14%22%0A%20%20%221%2C5%2C10%2C14%22-%3E%222%2C6%2C11%2C15%22%5Blabel%3D%22c%22%5D%0A%20%20%222%2C6%2C11%2C15%22-%3E%223%2C7%2C12%2C16%22%5Blabel%3D%22a%22%5D%0A%20%20%223%2C7%2C12%2C16%22-%3E%224%2C8%22%5Blabel%3D%22t%22%5D%0A%20%20%223%2C7%2C12%2C16%22-%3E%2213%2C17%22%5Blabel%3D%22r%22%5D%0A%20%20%224%2C8%22-%3E9%5Blabel%3D%22s%22%5D%0A%20%20%2213%2C17%22-%3E18%5Blabel%3D%22s%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)

## 2.6 在下面这个自动机中找出两个等价的状态，并合并它们产生一个识别相同语言且较小的自动机。重复这个过程直到没有等价状态为止。
`答案：`

- s1 = {{1,2,4,5,6,7,8},{3}}
- s2 = {{1,5,7},{2,4,8},{6},{3}}
- s3 = {{1,5},{7},{2,8},{4,6}}
- s4 = {{1,5},{7},{2,8},{4,6}}

因为 s3==s4 所以终止迭代

<!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];"1,5";"2,8";7;"4,6"}
  {node[shape=doublecircle];3}
  start->"1,5"
  "1,5"->"2,8"[label="0"]
  "1,5"->"4,6"[label="1"]
  "2,8"->7[label="0"]
  "2,8"->3[label="1"]
  7->7[label="0"]
  7->"1,5"[label="1"]c
  "4,6"->7[label="1"]
  "4,6"->3[label="0"]
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B%221%2C5%22%3B%222%2C8%22%3B7%3B%224%2C6%22%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B3%7D%0A%20%20start-%3E%221%2C5%22%0A%20%20%221%2C5%22-%3E%222%2C8%22%5Blabel%3D%220%22%5D%0A%20%20%221%2C5%22-%3E%224%2C6%22%5Blabel%3D%221%22%5D%0A%20%20%222%2C8%22-%3E7%5Blabel%3D%220%22%5D%0A%20%20%222%2C8%22-%3E3%5Blabel%3D%221%22%5D%0A%20%207-%3E7%5Blabel%3D%220%22%5D%0A%20%207-%3E%221%2C5%22%5Blabel%3D%221%22%5D%0A%20%20%224%2C6%22-%3E7%5Blabel%3D%221%22%5D%0A%20%20%224%2C6%22-%3E3%5Blabel%3D%220%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)

## *2.7 任何接收至少一个字符串的 DFA 都能转换为一个正则表达式。将习题 2.3c 的 DFA 转化为正则表达式。**提示:** 首先假装状态1是初态。然后，编写一个通到状态 2 并返回到状态 1 的正则表达式和一个类似的通到状态 0 并返回到状态 1 的正则表达式。或者查看 Hopcroft 和 Ullman[1979]一书中定理 2.4 关于此算法的论述。
`0*(1|(01*0)*)1)*`

## 2.9 一个基于 DFA 的解释型词法分析器使用以下两张表。
- edges 以状态和输入符号为索引，产生一个状态号。
- final 以状态为索引，返回 0 或一个动作号。

从下面这个词法规范开始：
```
(aba)+      (action 1);
(a(b*)a)    (action 2);
(a|b)       (action 3);
```
为一个词法分析器生成 edges 和 final 表。

`答案：`

先通过该题词法规范得出 NFA:
<!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];1;2;3;4;5;7;8;9}
  {node[shape=doublecircle];6;10;11}
  start->1
  1->2[label="ε"]
  1->3[label="ε"]
  1->4[label="ε"]
  2->5[label="a"]
  3->6[label="a|b"]
  4->7[label="a"]
  5->8[label="b"]
  7->9[label="ε"]
  8->10[label="a"]
  9->9[label="b"]
  10->2[label="ε"]
  9->11[label="a"]
  
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B2%3B3%3B4%3B5%3B7%3B8%3B9%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B6%3B10%3B11%7D%0A%20%20start-%3E1%0A%20%201-%3E2%5Blabel%3D%22%CE%B5%22%5D%0A%20%201-%3E3%5Blabel%3D%22%CE%B5%22%5D%0A%20%201-%3E4%5Blabel%3D%22%CE%B5%22%5D%0A%20%202-%3E5%5Blabel%3D%22a%22%5D%0A%20%203-%3E6%5Blabel%3D%22a%7Cb%22%5D%0A%20%204-%3E7%5Blabel%3D%22a%22%5D%0A%20%205-%3E8%5Blabel%3D%22b%22%5D%0A%20%207-%3E9%5Blabel%3D%22%CE%B5%22%5D%0A%20%208-%3E10%5Blabel%3D%22a%22%5D%0A%20%209-%3E9%5Blabel%3D%22b%22%5D%0A%20%2010-%3E2%5Blabel%3D%22%CE%B5%22%5D%0A%20%209-%3E11%5Blabel%3D%22a%22%5D%0A%20%20%0A%20%20rankdir%3D%22LR%22%0A%7D)

其中
- ***状态10: 表示 action1***
- ***状态11: 表示 action2***
- ***状态6: 表示 action3***


再将上面的 NFA 转成 DFA:

<!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];"1,2,3,4";"8,9";9;5;8;}
  {node[shape=doublecircle];"5,6,7,9";"2,10,11";11;"2,10";6}
  start->"1,2,3,4"
  "1,2,3,4"->"5,6,7,9"[label="a"]
  "1,2,3,4"->6[label="b"]
  "5,6,7,9"->11[label="a"]
  "5,6,7,9"->"8,9"[label="b"]
  "8,9"->"2,10,11"[label="a"]
  "8,9"->9[label="b"]
  "2,10,11"->5[label="a"]
  5->8[label="b"]
  8->"2,10"[label="a"]
  "2,10"->5[label="a"]
  9->9[label="b"]
  9->11[label="a"]
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B%221%2C2%2C3%2C4%22%3B%228%2C9%22%3B9%3B5%3B8%3B%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B%225%2C6%2C7%2C9%22%3B%222%2C10%2C11%22%3B11%3B%222%2C10%22%3B6%7D%0A%20%20start-%3E%221%2C2%2C3%2C4%22%0A%20%20%221%2C2%2C3%2C4%22-%3E%225%2C6%2C7%2C9%22%5Blabel%3D%22a%22%5D%0A%20%20%221%2C2%2C3%2C4%22-%3E6%5Blabel%3D%22b%22%5D%0A%20%20%225%2C6%2C7%2C9%22-%3E11%5Blabel%3D%22a%22%5D%0A%20%20%225%2C6%2C7%2C9%22-%3E%228%2C9%22%5Blabel%3D%22b%22%5D%0A%20%20%228%2C9%22-%3E%222%2C10%2C11%22%5Blabel%3D%22a%22%5D%0A%20%20%228%2C9%22-%3E9%5Blabel%3D%22b%22%5D%0A%20%20%222%2C10%2C11%22-%3E5%5Blabel%3D%22a%22%5D%0A%20%205-%3E8%5Blabel%3D%22b%22%5D%0A%20%208-%3E%222%2C10%22%5Blabel%3D%22a%22%5D%0A%20%20%222%2C10%22-%3E5%5Blabel%3D%22a%22%5D%0A%20%209-%3E9%5Blabel%3D%22b%22%5D%0A%20%209-%3E11%5Blabel%3D%22a%22%5D%0A%20%20%20rankdir%3D%22LR%22%0A%7D)

其中
- ***状态(5,6,7,9): 表示 action3***
- ***状态11: 表示 action2***
- ***状态(2,10,11): 表示 [action1,action2]***
- ***状态6: 表示 action3***
- ***状态(2,10): 表示 action1***

 调整状态序号：
<!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];1;4;6;7;9;}
  {node[shape=doublecircle];2;5;8;10;3}
  start->1
  1->2[label="a"]
  1->3[label="b"]
  2->8[label="a"]
  2->4[label="b"]
  4->5[label="a"]
  4->6[label="b"]
  5->7[label="a"]
  7->9[label="b"]
  9->10[label="a"]
  10->7[label="a"]
  6->6[label="b"]
  6->8[label="a"]
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B4%3B6%3B7%3B9%3B%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B2%3B5%3B8%3B10%3B3%7D%0A%20%20start-%3E1%0A%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%201-%3E3%5Blabel%3D%22b%22%5D%0A%20%202-%3E8%5Blabel%3D%22a%22%5D%0A%20%202-%3E4%5Blabel%3D%22b%22%5D%0A%20%204-%3E5%5Blabel%3D%22a%22%5D%0A%20%204-%3E6%5Blabel%3D%22b%22%5D%0A%20%205-%3E7%5Blabel%3D%22a%22%5D%0A%20%207-%3E9%5Blabel%3D%22b%22%5D%0A%20%209-%3E10%5Blabel%3D%22a%22%5D%0A%20%2010-%3E7%5Blabel%3D%22a%22%5D%0A%20%206-%3E6%5Blabel%3D%22b%22%5D%0A%20%206-%3E8%5Blabel%3D%22a%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)


其中
- ***状态2: 表示 action3***
- ***状态8: 表示 action2***
- ***状态5: 表示 [action1,action2]***
- ***状态3: 表示 action3***
- ***状态10: 表示 action1***

最小化此 DFA：

- s1 = {{1,4,6,7,9},{2,3,5,8,10}}
- s2 = {{1},{4,6},{7},{9},{2},{3,8},{5,10}}
- s3 = {{1},{4},{6},{7},{9},{2},{3,8},{5,10}}
- s4 = {{1},{4},{6},{7},{9},{2},{3,8},{5,10}}

因为 s3==s4 所以终止迭代

<!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];1;4;6;7;9;}
  {node[shape=doublecircle];2;"5,10";"3,8";"5,10";"3,8"}
  start->1
  1->2[label="a"]
  1->"3,8"[label="b"]
  2->"3,8"[label="a"]
  2->4[label="b"]
  4->"5,10"[label="a"]
  4->6[label="b"]
  "5,10"->7[label="a"]
  7->9[label="b"]
  9->"5,10"[label="a"]
  6->6[label="b"]
  6->"3,8"[label="a"]
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B4%3B6%3B7%3B9%3B%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B2%3B%225%2C10%22%3B%223%2C8%22%3B%225%2C10%22%3B%223%2C8%22%7D%0A%20%20start-%3E1%0A%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%201-%3E%223%2C8%22%5Blabel%3D%22b%22%5D%0A%20%202-%3E%223%2C8%22%5Blabel%3D%22a%22%5D%0A%20%202-%3E4%5Blabel%3D%22b%22%5D%0A%20%204-%3E%225%2C10%22%5Blabel%3D%22a%22%5D%0A%20%204-%3E6%5Blabel%3D%22b%22%5D%0A%20%20%225%2C10%22-%3E7%5Blabel%3D%22a%22%5D%0A%20%207-%3E9%5Blabel%3D%22b%22%5D%0A%20%209-%3E%225%2C10%22%5Blabel%3D%22a%22%5D%0A%20%206-%3E6%5Blabel%3D%22b%22%5D%0A%20%206-%3E%223%2C8%22%5Blabel%3D%22a%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)


其中
- ***状态2: 表示 action3***
- ***状态5,10: 表示 [action1,action2]***
- ***状态3,8: 表示 [action2,action3]***

 调整状态序号：

 <!-- This is the original graph
digraph G {
  start[label= "", shape=none,height=.0,width=.0]
  {node[shape=circle];1;3;5;6;8;}
  {node[shape=doublecircle];2;4;7;4;7}
  start->1
  1->2[label="a"]
  1->7[label="b"]
  2->7[label="a"]
  2->3[label="b"]
  3->4[label="a"]
  3->5[label="b"]
  4->6[label="a"]
  6->8[label="b"]
  8->4[label="a"]
  5->5[label="b"]
  5->7[label="a"]
  rankdir="LR"
}
-->

![digraph](https://g.gravizo.com/svg?digraph%20G%20%7B%0A%20%20start%5Blabel%3D%20%22%22%2C%20shape%3Dnone%2Cheight%3D.0%2Cwidth%3D.0%5D%0A%20%20%7Bnode%5Bshape%3Dcircle%5D%3B1%3B3%3B5%3B6%3B8%3B%7D%0A%20%20%7Bnode%5Bshape%3Ddoublecircle%5D%3B2%3B4%3B7%3B4%3B7%7D%0A%20%20start-%3E1%0A%20%201-%3E2%5Blabel%3D%22a%22%5D%0A%20%201-%3E7%5Blabel%3D%22b%22%5D%0A%20%202-%3E7%5Blabel%3D%22a%22%5D%0A%20%202-%3E3%5Blabel%3D%22b%22%5D%0A%20%203-%3E4%5Blabel%3D%22a%22%5D%0A%20%203-%3E5%5Blabel%3D%22b%22%5D%0A%20%204-%3E6%5Blabel%3D%22a%22%5D%0A%20%206-%3E8%5Blabel%3D%22b%22%5D%0A%20%208-%3E4%5Blabel%3D%22a%22%5D%0A%20%205-%3E5%5Blabel%3D%22b%22%5D%0A%20%205-%3E7%5Blabel%3D%22a%22%5D%0A%20%20rankdir%3D%22LR%22%0A%7D)

- ***状态2: 表示 action3***
- ***状态4: 表示 [action1,action2]***
- ***状态7: 表示 [action2,action3]***

由上图得出 edges 表：
| |a|b|
|-|-|-|
|1|2|7|
|2|7|3|
|3|4|5|
|4|6| |
|5|7|5|
|6| |8|
|7| | |
|8|4| |


final 表：
|1|2|3|4|5|6|7|8|
|-|-|-|-|-|-|-|-|
|0|3|0|1,2|0|0|2,3|0|


然后给该词法分析器分析字符串 abaabbaba 的每一步。注意，一定要给出此词法分析器重要的内部变量的值，该词法分析器将被反复调用以获得后继的单词。

|当前状态|字符索引|当前字符|备注|
|-|-|-|-|
|1|0| |初始状态默认为1|
|2|1|a|终态|
|3|2|b| |
|4|3|a|终态|
|6|4|a| |
|8|5|b| |
| |6|b|错误状态，因此取最后一个终态4，字符索引为3|

根据上表 字符串 `aba` 最终为状态4，再通过final表可得 `aba` 为 action1。

从字符索引3开始继续向后识别
|当前状态|字符索引|当前字符|备注|
|-|-|-|-|
|1|3| |初始状态默认为1|
|2|4|a|终态|
|3|5|b| |
|5|6|b| |
|7|7|a|终态|
| |8|b|错误状态，因此取最后一个终态7，字符索引为7|

根据上表 字符串 `abba`  最终为状态7，再通过final表可得 `abba` 为 action2。

从字符索引7开始继续向后识别
|当前状态|字符索引|当前字符|备注|
|-|-|-|-|
|1|7| |初始状态默认为1|
|7|8|b|终态|
| |9|a|错误状态，因此取最后一个终态7，字符索引为8|

根据上表 字符串 `b`  最终为状态7，再通过final表可得 `b` 为 action2。

从字符索引7开始继续向后识别
|当前状态|字符索引|当前字符|备注|
|-|-|-|-|
|1|8| |初始状态默认为1|
|2|9|a|终态|

最后的字符串 `a` 状态为2，通过final表可得 `a` 为 action3。

所以词法分析 `abaabbaba` 结果为：

|aba|abba|b|a|
|-|-|-|-|
|action1|action2|action2|action3|