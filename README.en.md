# maidata-rs

[简体中文](README.md)

Parses [the `maidata.txt` format][format] of the [simai] application, popular
in the maimai mapping community.

[simai]: https://w.atwiki.jp/simai/
[format]: https://w.atwiki.jp/simai/pages/25.html

Currently very much WIP, expect a lot of breakages. Don't use in production.

## Supported `maidata.txt` features

General format features:

* [x] basic metadata fields
* [ ] comments `||xxx\n`
* [ ] escape sequences `\＆ \＋ \％ \￥`
* [ ] active message fields

Map definition instructions:

* [x] BPM spec `(float)`
* [x] beat divisor spec `{4}`
    - [x] normal spec `{4}`
    - [x] absolute duration spec `{#0.25}`
* [x] end mark `E`
* [x] TAP `B,`
    - [x] simplified BOTH/EACH TAP form (`16` `38` etc.; `123` and such are also allowed)
    - [x] BREAK modifier `Bb,`
    - [x] EX NOTES modifier `Bx,` (3simai)
    - [x] star-shape modifier `B$,` `B$$,`
    - [x] SLIDE head modifier `B@,` `B?xE,` `B!xE,`
* [x] HOLD `Bh[duration],`
    - [x] BREAK modifier `Bbh[duration],` (3simai)
    - [ ] no-duration form `Bh,`
* [x] SLIDE `FxE[duration],`
    - [x] all track shapes `- ^ < > v p q s z pp qq V w`
    - [x] multiple tracks sharing one start `1-3[4:1]*-4[4:1]`
    - [x] chaining tracks `1-4q7-2[1:2]` (3simai)
    - [x] BREAK modifier `1-2-3[2:1]b` (3simai)
    - [x] duration specs
        - [x] `[160#2.0]`
        - [x] `[3.0##1.5]`
        - [x] `[3.0##4:1]`
        - [x] `[3.0##160#4:1]`
* [x] TOUCH `T,` (3simai)
    - [x] FIREWORK modifier `Tf,`
* [x] TOUCH HOLD `Th[duration],` (3simai)
    - [ ] no-duration form `Th,`
* [x] BOTH/EACH `note/note,`
    - [x] arbitrary number of concurrent notes allowed (3simai)
    - [ ] pseudo EACH ``1`2`3`4,``

`duration` format:

* [x] normal duration spec `[x:y]`
* [x] absolute duration specs `[#sec]`
* [x] normal duration spec with BPM `[bpm#x:y]`
