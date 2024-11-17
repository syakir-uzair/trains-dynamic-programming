# bigpay-test-trains

## Setup (JS)

1. Install nvm https://github.com/nvm-sh/nvm.
1. Run `nvm i` to install NodeJS.
1. Run `npm i` to install packages.

## Test (JS)

1. Run `npm start` to compile and execute the tests.

## Setup (Rust)

1. Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install Rust.

## Test (Rust)

1. Run `cargo run` to compile and execute the tests.

## Solution

1. The graph calculation/navigation is based on Dijkstra's Algorithm with priority queue.
1. For navigation, it is based on dynamic programming, where all combinations will calculated and calculated state (trains' location and packages picked up / delivered by which train) will be cached.
1. With the same distance, routes with the least trains will be picked.
