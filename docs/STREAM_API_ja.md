# Stream API 設計仕様書

本書では、`umatoi` におけるストリームベースの通知システムの設計と実装について説明します。

## 1. コンセプト解説

### 1.1 Stream API とは？
Rust における `Stream` とは、**「非同期版のイテレータ (Iterator)」** です。
通常のイテレータが次の値を即座に（同期的に）返すのに対し、ストリームは時間の経過とともに発生する値を非同期に提供します。

- **Iterator**: `next()` を呼ぶと、すぐに値が返ってくる。
- **Stream**: `next().await` を呼ぶと、次のイベント（センサーデータなど）が届くまで待機し、届いた時点で値を返す。

toio のような「いつ届くかわからないセンサー通知」を扱うには、この非同期ストリームが最適です。

### 1.2 `tokio::sync::broadcast` とは？
`broadcast` チャネルは、「1対多」の通信メカニズムです。

- **MPSC (複数プロデューサー・単一コンシューマー)**: 送信者はたくさん、受信者は1人だけ。
- **Broadcast (ブロードキャスト)**: 送信者は1人、**受信者は何人でもOK**。

`umatoi` では、キューブから届いた一つのイベントを、アプリケーション内の複数の場所（例：画面表示タスク、ログ記録タスク、制御タスクなど）で同時に受け取れるようにこの仕組みを採用しています。

## 2. システム概要
従来のコールバック方式を廃止し、この Stream ベースの API に置き換えました。これにより、Rust の標準的な非同期パターンを使って、toio™ コア キューブの通知を直感的に処理できるようになります。

## 3. 主要コンポーネント

### 3.1 CubeEvent 列挙型
すべての通知は `CubeEvent` 列挙型に統合されます。

```rust
pub enum CubeEvent {
    Id(IdInformation),                // 座標・タグ情報
    Button(ButtonInformation),        // ボタン状態
    Battery(BatteryInformation),      // バッテリー残量
    Sensor(SensorInformation),        // 各種センサー情報
    Motor(MotorInformation),          // モーター応答・速度
    Unknown { uuid: Uuid, data: Vec<u8> }, // 未知の通知
}
```

### 3.2 NotificationManager
内部で `tokio::sync::broadcast` を管理します。複数の購読者が、互いに干渉することなく並列にキューブの状態を監視できます。

### 3.3 CubeInterface トレイト
特定のストリームを取得するためのメソッドを提供します。

```rust
pub trait CubeInterface {
    async fn event_stream(&self) -> Result<BoxStream<'static, CubeEvent>, ...>;
    async fn id_stream(&self) -> Result<BoxStream<'static, IdInformation>, ...>;
    // ... 他にもボタン用やバッテリー用があります
}
```

## 4. 実装の詳細

### 4.1 通知ループとライフサイクル
`connect()` 時にバックグラウンドタスクが起動します。`CancellationToken` を導入しているため、`disconnect()` を呼んだ際やキューブのオブジェクトが破棄された際に、バックグラウンドタスクも即座に停止し、リソースリークを防ぎます。

## 5. 使用例

```rust
let mut cube = scanner.scan(1, Duration::from_secs(5)).await?;
cube.connect().await?;

// ID通知（座標など）のストリームを取得
let mut id_stream = cube.id_stream().await?;

// バックグラウンドタスクでストリームを処理
tokio::spawn(async move {
    while let Some(id) = id_stream.next().await {
        println!("現在座標: {:?}", id);
    }
});
```

## 6. メリット
- **並列処理**: 複数のタスクで異なるセンサーを同時に監視できる。
- **簡潔なコード**: 複雑なコールバック登録の代わりに、`while let` ループで直感的に書ける。
- **安全な終了処理**: キャンセルトークンにより、バックグラウンドタスクの寿命が適切に管理される。
