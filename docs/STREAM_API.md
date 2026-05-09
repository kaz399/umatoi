# Stream API Design Specification

This document describes the design and implementation of the Stream-based notification system in `umatoi`.

## 1. Concept

### 1.1 What is Stream API?
In Rust, a `Stream` is an **asynchronous version of an Iterator**.
While an `Iterator` provides the next value immediately (synchronously), a `Stream` yields values over time (asynchronously).

- **Iterator**: `next()` -> returns a value right now.
- **Stream**: `next().await` -> waits until the next event (e.g., sensor data) arrives.

This is perfect for robot notifications because data arrives at unpredictable intervals.

### 1.2 What is `tokio::sync::broadcast`?
The `broadcast` channel is a "One-to-Many" communication mechanism.

- **MPSC (Multi-Producer, Single-Consumer)**: Many senders, but only one receiver.
- **Broadcast**: One sender (the Cube), but **multiple receivers**.

In `umatoi`, the Cube broadcasts a single event, and multiple parts of your code (e.g., a GUI task and a logging task) can receive the exact same event simultaneously.

## 2. Overview of the New System
The previous callback-based system has been replaced with this Stream-based API. It allows you to use standard async patterns to handle toio™ core cube notifications.

## 3. Core Components

### 3.1 CubeEvent Enum
All notifications are unified into the `CubeEvent` enum.

```rust
pub enum CubeEvent {
    Id(IdInformation),
    Button(ButtonInformation),
    Battery(BatteryInformation),
    Sensor(SensorInformation),
    Motor(MotorInformation),
    Unknown { uuid: Uuid, data: Vec<u8> },
}
```

### 3.2 NotificationManager
Internally uses `tokio::sync::broadcast` to allow multiple subscribers to monitor the cube state in parallel without interfering with each other.

### 3.3 CubeInterface Trait
Provides methods to get specific streams.

```rust
pub trait CubeInterface {
    async fn event_stream(&self) -> Result<BoxStream<'static, CubeEvent>, ...>;
    async fn id_stream(&self) -> Result<BoxStream<'static, IdInformation>, ...>;
    // ...
}
```

## 4. Implementation Details

### 4.1 Notification Loop & Lifecycle
When `connect()` is called, a background task starts. We use a `CancellationToken` to ensure this task stops immediately when `disconnect()` is called or the cube object is dropped, preventing resource leaks.

## 5. Usage Example

```rust
let mut cube = scanner.scan(1, Duration::from_secs(5)).await?;
cube.connect().await?;

// Get a stream of position IDs
let mut id_stream = cube.id_stream().await?;

// Process the stream in a background task
tokio::spawn(async move {
    while let Some(id) = id_stream.next().await {
        println!("Position: {:?}", id);
    }
});
```

## 6. Benefits
- **Parallelism**: Multiple tasks can monitor different sensors concurrently.
- **Clean Code**: Use `while let Some(...) = stream.next().await` instead of complex callback registrations.
- **Safe Lifecycle**: Background tasks are automatically managed via cancellation tokens.
