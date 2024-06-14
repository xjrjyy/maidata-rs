# maidata-rs

[English](README.en.md)

解析 [simai] 的 [`maidata.txt` 文件格式][format] 的库。simai 是 maimai 自制谱谱师群体所常用的工具。

[simai]: https://w.atwiki.jp/simai/
[format]: https://w.atwiki.jp/simai/pages/25.html

项目目前处于很早的阶段，尚未施工完成，不能保证兼容，慎用。

## 支持的 `maidata.txt` 特性

文件格式的通用特性:

* [x] 基本的元数据字段
* [ ] 注释 `||xxx\n`
* [ ] 转义序列 `\＆ \＋ \％ \￥`
* [ ] active message 字段

谱面定义指令:

* [x] BPM 设置 `(float)`
* [x] x 分音符设置 `{4}`
    - [x] 通常形式 `{4}`
    - [x] 绝对长度时值形式 `{#0.25}`
* [x] 谱面结束标记 `E`
* [x] TAP `B,`
    - [x] BOTH/EACH TAP 简化形式 (`16` `38` 等；`123` 之类的多押也允许)
    - [x] BREAK 修饰符 `Bb,`
    - [x] EX NOTES 修饰符 `Bx,` (3simai)
    - [ ] 强制星星形状修饰符 `B$,` `Bb$, B$b,` `B$$,`
* [x] HOLD `Bh[duration],`
    - [x] BREAK 修饰符 `Bbh[duration],` (3simai)
* [x] SLIDE `FxE[duration],`
    - [x] 所有的星星轨迹形状 `- ^ < > v p q s z pp qq V w`
    - [x] 共享一个星星头的多条轨迹 `1-3[4:1]*-4[4:1]`
    - [x] 链接星星 `1-4q7-2[1:2]` (3simai)
    - [x] BREAK 修饰符 `1-2-3[2:1]b` (3simai)
    - [ ] 特殊时值形式
        - [x] `[3.0##1.5]`
        - [ ] `[3.0##4:1]`
        - [ ] `[3.0##160#4:1]`
* [x] TOUCH `B,` (3simai)
    - [x] FIREWORK 修饰符 `Bf,`
* [x] TOUCH HOLD `Bh[duration],` (3simai)
* [x] BOTH/EACH `note/note,`
    - [x] 支持任意个数的多押 (3simai)

`duration` 格式：

* [x] 通常时值形式 `[x:y]`
* [x] 绝对长度时值形式 `[#sec]`
* [ ] BPM 通常时值形式 `[bpm#x:y]`
