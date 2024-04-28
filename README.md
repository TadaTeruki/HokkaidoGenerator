# Hokkaido Generator - 北海道ジェネレータ -

Website: **[hokkaidogenerator.peruki.dev/](https://hokkaidogenerator.peruki.dev/)**

実在しない北海道の市街を自動生成します。

![HokkaidoGenerator](https://github.com/TadaTeruki/HokkaidoGenerator/assets/69315285/40685a5f-f91d-48a5-8551-54113ee9044d)

![Screenshot](https://github.com/TadaTeruki/HokkaidoGenerator/assets/69315285/a8409246-99fc-4e7d-acbe-bca2adb36e93)

## ビルド方法

```
$ cd frontend
$ bun i
$ bun run dev
```

シミュレータ部分の更新は以下のコマンドで行います。

```
$ cd generator
$ make
```

## 技術構成

Rustで開発したシミュレータをWebAssemblyビルドし、TypeScript側で表示する形式。

### フロントエンド

言語: TypeScript <br>
開発環境: Svelte + SvelteKit <br>

地図の表示にはMaplibre GL JSを利用。

## 手法構成

- **区画・交通網生成: 拡張L-system (に由来する生成アルゴリズム)**<br>
Parish and Müller[^pm] に基づいた手法を、Sean Barrett[^barrett] による実装方針に合わせ実装しているもの。<br>
実装の流れはRobin(phiresky)[^phi] の資料を参考としている。<br>
source: https://github.com/TadaTeruki/street-engine

[^pm]: Parish, Yoav I. H., and Pascal Müller. 2001. “Procedural Modeling of Cities.” In Proceedings of the 28th Annual Conference on Computer Graphics and Interactive Techniques, 301–8. SIGGRAPH ’01. New York, NY, USA: ACM. https://doi.org/10.1145/383259.383292. 

[^barrett]: Sean Barrett. 2008. “L-Systems Considered Harmful.” 2008. http://nothings.org/gamedev/l_systems.html. 

[^phi]: https://phiresky.github.io/procedural-cities/

- **地名生成: Markov連鎖**<br>
既存地名を漢字ごとに分け、発音の繋がりで新しいパターンを組み上げる独自実装。<br>
source: https://github.com/TadaTeruki/name-engine

- **地形生成: Landscape Evolution Model**<br>
Salève model[^analytical] を、 Cordonnier et al.[^large] の手法を取り入れつつ実装。<br>
source: https://github.com/TadaTeruki/fastlem

[^analytical]: Steer, P.: Short communication: Analytical models for 2D landscape evolution, Earth Surf. Dynam., 9, 1239–1250, https://doi.org/10.5194/esurf-9-1239-2021, 2021.

[^large]: Guillaume Cordonnier, Jean Braun, Marie-Paule Cani, Bedrich Benes, Eric Galin, et al.. Large Scale Terrain Generation from Tectonic Uplift and Fluvial Erosion. Computer Graphics Forum, 2016, Proc. EUROGRAPHICS 2016, 35 (2), pp.165-175. ⟨10.1111/cgf.12820⟩. ⟨hal-01262376⟩

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

## ライセンス・権利表示

ライセンス: MPL-2.0

Copyright (c) 2024 Teruki TADA
