# Rust Timer Proc Macros

The Rust macros that used to estimate function call time cost.

## Install

- Clone this repositry

   ```shell 
   git clone https://github.com/li195111/qctimer_macros.git
   ```

### Add to your project's Cargo.toml

- Go to the project that you want to use `qctimer`
- Add dependencies as following

    ```toml
    [dependencies]
    qctimer_macros = { path = "< PATH / TO / qctimer_macros >" }
    ```

## How to use

```Rust
use qctimer_macros::timer;

#[timer]
fn function_you_want_estimate(...) {
    ...
}
```

### Async Use

```Rust
use qctimer_macros::async_timer;

#[async_timer]
async fn async_function_you_want_estimate(...) {
    ...
}
```

## Result in console

It will `println` following messages.

```Shell
function_you_want_estimate Time Cost: 3 ms.
another_function_you_want_estimate Time Cost: 637 µs.
```