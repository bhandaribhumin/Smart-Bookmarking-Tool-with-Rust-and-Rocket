## Install Rust

To install Rust, I will use a tool called rustup. You can install it by running the command:

```ruby
    shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### This will install rustup which is used for installing Rust and staying up to date with different versions of the language. After you run it and follow the instructions, you should have Rust installed.

You can double-check by running:

```ruby
         shell
         cargo --version
```

### Cargo is the official Rust package manager and we will use it to compile, build and run our Rust code.

Switch to Nightly Rust

# Run

```ruby
cargo run
```

Open Browser

> http://localhost:8000

# Navigate

Navigate to GitHub -

> http://localhost:8000/search?cmd=gh
> Navigate to GitHub -
> http://localhost:8000/search?cmd=gh%20bhandaribhumin/Toy-Robot-Simulator-ionic

## test

```ruby
cargo test -- test_construct_twitter_search_url
cargo test -- test_construct_twitter_profile_url
```

### ref

[Article](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/)
