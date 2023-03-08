# farm-operator

Example operator for Kubernetes in Rust.

[Tutorial.](https://metalbear.co/blog/writing-a-kubernetes-operator/controller/)

This repo contians 3 examples and a copy of example 1 in `./src`.

```bash
cargo run # ./

cargo run -p farm-operator-1 # ./example/step-1

cargo run -p farm-operator-2 # ./example/step-2

cargo run -p farm-operator-3 # ./example/step-3
```

* **farm-operator-1** - Minimum Kubernetes APIService
* **farm-operator-2** - Serves Llama resource
* **farm-operator-3** - Serves Llama & FarmPod resource
