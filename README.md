# Rust OTP Code Generator

Simple library for generating and verifying one-time passwords (OTP) using the HOTP and TOTP algorithms.

## How to install

First, you need to add this library into your project but sadly we only support github import method

```bash
otp = { git = "https://github.com/Sleepy4k/rust-otp-code", branch = "main" }
```

Add that line code into your 'Cargo.toml' file, And boom this library already on your project.

## How to use

### HMAC Tutorial

I am gonna show you how to use hmac otp, first import function in your code

```bash
use otp::make_hmac_otp;
```

After that you can write it on your code, for example i am gonna show you on common condition

```bash
let secret_key = "YourVerySecretKey";
let counter = 1; // change with your counter
let generated_otp = make_hmac_otp(secret_key, counter).unwrap();

println!("Your OTP Code is : {}", generated_otp);
```

### TOTP Tutorial

For totp you can implement it for otp code that need to verify if otp code is still valid,
as always you need to import function in your code

```bash
use otp::make_time_otp;
```

After that you can write it on your lovely code, i already write an example below

```bash
let secret_key = "YourVerySecretKey";
let time_step = 3600; // change with your time step config
let skew = 0;
let generated_otp = make_time_otp(secret.as_str(), time_step, skew).unwrap();

println!("Your OTP Code is : {}", generated_otp);
```

## Project Test

You can run some test for checking library function or result with cargo command

```bash
cargo test
```
