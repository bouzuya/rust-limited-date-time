# limited-date-time

## モジュール間の依存関係

![mod_deps 0.16.0](https://user-images.githubusercontent.com/1221346/139870635-4b1442c1-5e81-4a7f-95a3-da9176fdaf20.png)

## 実装メモ

- 制限が多いので `restricted-date-time` という名前に変えるかもしれない
  - formatter は提供せず RFC3339 にさらに制限を加えたものにする
  - 1970 年よりも過去、 9999 年より未来の日付を許容しない
    - `YYYY` の 4 桁固定とする
  - 2 桁の年を許容しない
    - `YYYY` の 4 桁固定とする
  - `-00:00` を許容しない
    - 不明な場合は省略する
  - date と time の区切り文字としての `' '` や `'t'` を許容しない
    - `'T'` 固定とする
  - UTC を表す文字としての `'z'` を許容しない
    - `'Z'` 固定とする
- `YearMonth::at_day(DayOfMonth) -> Result<LocalDate, _>` を追加しない
  - 理由は `YearMonth` から `LocalDate` への依存が生まれて相互依存となるのを避けるため
  - 依存の方向は `LocalDate` → `YearMonth` とする
  - 代替は `LocalDate::from_ymd(Year, Month, DayOfMonth) -> Result<LocalDate, _>` とする
  - `LocalDate` に `YearMonth` と `DayOfMonth` を引数に取るコンストラクタを追加しても良さそう
- `Instant::at_offset(TimeZoneOffset) -> Result<OffsetDateTime, _>` を追加しない
  - 理由は `YearMonth::at_day` と同様
  - 依存の方向は `OffsetDateTime` → `Instant` とする
- `OffsetDateTime::with_...(...)` を追加しない
  - 理由は `with` で指定した箇所以外も変更する必要が出ることによるあいまいさを避けるため
  - 例: `with_offset_same_instant` or `with_offset_same_local`
  - 例: `2021-03-31T00:00:00Z` に対しての `with_month(3)`
  - 代替は `OffsetDateTime::from_...` による再構築とする
- `Month::succ` の `&mut self` をやめるべきか
  - コピーが欲しい場合に面倒かもしれない
    - `Copy` を実装しているので単に新しいものを確保しても良いはず
    - `Month::succ(self) -> Month` のほうが良いかもしれない
  - Iterator 的な動きが欲しかった
    - ただ `for m in month {...}` は何を iterate するのかよく分からない
    - `for m in m..=Month::MAX` や `for m in (Month::MIN..=m).rev()` のほうが自然だ
      - この iterate だと当月が入るので翌月から走査したい場合はどうするのか
        - `succ` のようなものが欲しい (ループ)
    - どの方向に走査するのかが決まらない
      - `rev` は DoubleEndedIterator を実装しないといけないが実装できない (たぶん)
      - `successor` と `predecessor` を提供する？
        <https://doc.rust-lang.org/std/iter/trait.Step.html>
  - 12 月の次は None か 1 月か
    - 数字では 12 の次は 13 だが、月なら 1 だ
    - `Month::next(self) -> Month`
      - 失敗しない、常に次の月を返せる
    - 次の年の 1 月なのでループしているわけではない？
  - n ヶ月後の取得をどう表現する
- Days の利用場面を増やす - 期間として見て量 (日数) を返す
  - DayOfMonth -> Days はあり
    - 必要なさそうだけど 1 日だと思う
  - Month -> Days はなし
    - Year がついていないと日数は確定しない
  - YearMonth -> Days はあり
    - epoch からの日数などと誤読する？
      - diff や range で提供すると良さそう
    - ある年のある月の日数として読めそう
  - Year -> Days はあり
    - YearMonth -> Days と同様
  - Hour -> Days はなし
    - 端数
  - Minute -> Days はなし
    - Hour と同様
  - Second -> Days はなし
    - Hour と同様
  - Date -> Days はひとまずなし
    - 曖昧さがある
    - 基本的には 1 日のような気がする
    - 名前次第で Year::days, YearMonth::days のショートカットになる
      - 代替: Date::year から Year::days
      - 代替: Date::year_month から YearMonth::days
    - ショートカットのようなメソッドを利便性のために追加すべきか？
      - まだ提供していない
      - 提供すると混雑しそう
      - ひとまずなし
- `TimeZoneOffset::system_default` は実装しない
  - Rust の std の機能では実装できなさそう
  - chrono は libc で実装しているみたい
- 依存の方向がわかりにくくなってきた
  - ひとまず mod_deps.dot を作成した
  - メンテナンスに失敗しそう
  - 自動生成できると良さそう
