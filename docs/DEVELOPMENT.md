# Workflow Telemetry Rust - Development Guide

## プロジェクト概要

GitHub Actionsワークフローの実行中にCPUとメモリの使用状況を監視し、グラフ付きレポートを生成するツール。

元々はMermaidでグラフを作成する予定だったが、Mermaidでは限界があったため、`charts-rs`に切り替えた。

## アーキテクチャ

### 1. データ収集フェーズ

**バイナリ**: `telemetry` (Rust)

- バックグラウンドプロセスとして起動
- `/proc/stat`, `/proc/meminfo`からデータを収集
- 指定間隔でデータポイントを記録
- 終了時にJSON形式でデータを保存 (`/tmp/telemetry_data.json`)

**環境変数**:
- `TELEMETRY_INTERVAL`: データ収集間隔（秒）デフォルト5秒
- `TELEMETRY_ITERATIONS`: 最大収集回数 デフォルト60回

**出力**:
```json
{
  "cpu": [
    {"time": 1234567890, "total_load": 5.2, "user_load": 3.1, "system_load": 2.1},
    ...
  ],
  "memory": [
    {"time": 1234567890, "usage_percent": 45.0, "used_mb": 2048, "total_mb": 4096},
    ...
  ]
}
```

### 2. グラフ生成フェーズ

**コマンド**: `./telemetry --generate-svg <json_file>`

- JSONデータを読み込み
- `charts-rs`でSVG生成
- `charts-rs::svg_to_png()`でPNGに変換
- `cpu-usage.png`, `memory-usage.png`を出力

**重要ポイント**:
- `charts-rs`のPNG生成には`image-encoder` featureが必要
- `Cargo.toml`: `charts-rs = { version = "0.3.27", features = ["image-encoder"] }`
- `svg_to_png()`関数を使用（`.png()`メソッドは存在しない）

### 3. レポート表示フェーズ

**場所**: GitHub Actions Step Summary

- ワークフローからJSONを解析して統計を計算
- PNGをリポジトリにコミット (`docs/charts/`)
- Summaryに絶対URLで画像を表示

## 実装の難所と解決策

### 問題1: GitHub Step SummaryにSVG/PNGが表示されない

**試したこと**:
- ❌ インラインSVG埋め込み → 表示されない
- ❌ Base64エンコードしたSVG (`data:image/svg+xml;base64,...`) → 表示されない
- ❌ Base64エンコードしたPNG (`data:image/png;base64,...`) → 表示されない
- ❌ 相対パス (`docs/charts/cpu-usage.png`) → 404エラー
- ✅ **絶対URL** (`https://raw.githubusercontent.com/{owner}/{repo}/main/docs/charts/cpu-usage.png`)

**解決策**: 
画像をリポジトリにコミットして、絶対URLで参照する。

### 問題2: バックグラウンドプロセスからGITHUB_STEP_SUMMARYに書き込めない

**試したこと**:
- ❌ Rustバイナリから直接`$GITHUB_STEP_SUMMARY`に書き込み → ファイルは書けるが、Summaryに表示されない

**原因**: 
バックグラウンドプロセスから書き込んだファイルはGitHub Actionsが認識しない可能性がある。

**解決策**: 
ワークフローのステップから直接`>> $GITHUB_STEP_SUMMARY`で書き込む。

### 問題3: SIGTERMでシグナルハンドラーが呼ばれない

**試したこと**:
- ❌ `ctrlc`クレートでSIGTERMハンドラー → `sleep`中にシグナルを受け取ると無視される

**解決策**: 
シグナルを使わず、プロセスを自然終了させる方式に変更。
- 最大反復回数を設定 (`TELEMETRY_ITERATIONS`)
- ワークフローで終了を待つ

## ワークフロー構成

```yaml
permissions:
  contents: write  # 重要: チャート画像をコミットするため必要

jobs:
  test-telemetry:
    steps:
      # 1. telemetryをバックグラウンドで起動
      - name: Start telemetry monitoring
        run: |
          TELEMETRY_INTERVAL=2 TELEMETRY_ITERATIONS=15 ./telemetry > /tmp/telemetry.log 2>&1 &
          echo $! > /tmp/telemetry.pid

      # 2. ワークロード実行
      - name: Simulate workload
        run: |
          # 実際のビルド・テストなど

      # 3. telemetryの終了を待つ
      - name: Wait for telemetry to complete naturally
        run: |
          PID=$(cat /tmp/telemetry.pid)
          # プロセスが終了するまで待機（最大120秒）

      # 4. PNG生成
      - name: Generate SVG Charts
        run: |
          ./telemetry --generate-svg /tmp/telemetry_data.json
          mkdir -p docs/charts
          cp *.png docs/charts/
          git commit & push  # [skip ci]を付けて無限ループ回避

      # 5. Summaryに表示
      - name: Display Summary
        run: |
          jq で統計計算
          echo "![CPU](https://raw.githubusercontent.com/...)" >> $GITHUB_STEP_SUMMARY
```

## ビルド方法

```bash
# Linux向けビルド（macOSから）
cargo zigbuild --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/workflow-telemetry-rust telemetry

# バイナリ更新してコミット
git add telemetry
git commit -m "Update telemetry binary"
git push
```

## 参考実装

- [catchpoint/workflow-telemetry-action](https://github.com/catchpoint/workflow-telemetry-action)
  - 外部API (`api.globadge.com`) を使ってグラフ生成
  - 返されたURLをSummaryに埋め込む方式

## 次のステップ候補

1. **複数ワークフローへの対応**
   - 現在は固定の15回×2秒間隔
   - ワークフローの長さに応じて動的に調整

2. **統計の改善**
   - P95, P99などのパーセンタイル
   - 時系列での異常検知

3. **グラフの改善**
   - テーマ選択（dark/light）
   - 複数メトリクスを1つのグラフに
   - インタラクティブなグラフ（Plotlyなど）

4. **外部APIの利用**
   - QuickChart.io などの無料サービス
   - リポジトリコミット不要になる

5. **GitHub Action化**
   - composite actionとして公開
   - `uses: ke-kawai/workflow-telemetry-rust@v1`で使えるように

## トラブルシューティング

### グラフが表示されない

1. `docs/charts/`にPNGファイルが存在するか確認
2. コミットログに"Update telemetry charts"があるか確認
3. URLが正しいか確認（`https://raw.githubusercontent.com/...`)
4. 画像が最新のコミットのものか確認（ブラウザキャッシュクリア）

### データが収集されない

1. `/tmp/telemetry.log`を確認
2. プロセスが起動しているか確認 (`ps aux | grep telemetry`)
3. `/tmp/telemetry_data.json`が生成されているか確認

### ビルドエラー

- `charts-rs`の`image-encoder` featureが有効か確認
- `serde_json`が依存関係に含まれているか確認
- `CpuStats`, `MemoryStats`に`Deserialize` traitが実装されているか確認

## 技術スタック

- **言語**: Rust 2021 edition
- **グラフ**: charts-rs 0.3.27 (image-encoder feature)
- **シリアライズ**: serde, serde_json
- **ビルド**: cargo-zigbuild (クロスコンパイル用)
- **CI**: GitHub Actions
