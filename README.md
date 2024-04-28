# Hokkaido Generator - 北海道ジェネレータ -

![HokkaidoGenerator](https://github.com/TadaTeruki/HokkaidoGenerator/assets/69315285/40685a5f-f91d-48a5-8551-54113ee9044d)

## 技術構成

## 概要

- **区画・交通網: 拡張L-system (に由来する生成アルゴリズム)**<br>
Parish and Müller[^pm] に基づいた手法を、Sean Barrett[^barrett] による実装方針に合わせ実装しているもの。<br>
source: https://github.com/TadaTeruki/street-engine

[^pm]: Parish, Yoav I. H., and Pascal Müller. 2001. “Procedural Modeling of Cities.” In Proceedings of the 28th Annual Conference on Computer Graphics and Interactive Techniques, 301–8. SIGGRAPH ’01. New York, NY, USA: ACM. https://doi.org/10.1145/383259.383292. 

[^barrett]: Sean Barrett. 2008. “L-Systems Considered Harmful.” 2008. http://nothings.org/gamedev/l_systems.html. 

- **地名: Markov連鎖**<br>
既存地名を漢字ごとに分け、発音の繋がりで新しいパターンを組み上げる独自実装。<br>
source: https://github.com/TadaTeruki/name-engine

- **地形: Landscape Evolution Model**<br>
source: https://github.com/TadaTeruki/fastlem

## 地名データセットについて

[CSV file](https://github.com/TadaTeruki/HokkaidoGenerator/blob/main/dogen-frontend/static/dataset/placenames.csv)

地名の生成にあたり、
以下のフォーマットに基づくCSV形式のデータセットを用いている。

```
[漢字],[読み(ローマ字)],[各漢字に対応する読み]
```

各地名は、『北海道の地名』[^1] を参照し、アイヌ語由来の地名を中心に抜粋。なお、漢字に対応する読みの分け方は、本データ作成者の判断に基づく。

#### 例

```
琴似,kotoni,琴_koto:似_ni
発寒,hassamu,発_has:寒_samu
手稲,teine,手_te:稲_ine
```

[^1]: 山田秀三. 北海道の地名. 草風館, 2000.

## 権利表記

Copyright (c) 2024 Teruki TADA
