# wait_timer

Crate for more accurate wait times than `Delay` using `rp2040_hal::timer::Timer`. 
In addition to `wait` function that waits for a specified number of seconds, 
`gate` function is implemented that keeps the time until it is re-executed constant.

## How to use

1. Create `rp2040_hal::timer::Timer`.

```rust
let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS);
```

2. Create `wait_timer::Wait`.

```rust
let wait = Wait::new(&timer);
```

3. Use `wait` and/or `gate` methods.

## Methods

### `wait` functions

More accurate waiting times than Delay.

```rust
wait.wait_us(1_000_000);
wait.wait_ms(1_000);
wait.wait_sec(1);
```

### `gate` functions

Wait a specified number of seconds since the last execution.

```rust
loop {
    wait.gate_sec(1); // A
    // ...
    wait.gate_ms(500); // B
    // ...
}
```

1. A is ignored the first time.
2. The process from A to B is executed in 500 milliseconds.
3. Processes from B onwards (B to A) are executed in a second.