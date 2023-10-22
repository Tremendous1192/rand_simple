# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

```rand_simple``` is a Rust crate designed for efficient generation of pseudo-random numbers based on the Xorshift160 algorithm. It offers the following key features:

* Xorshift160 Algorithm: ```rand_simple``` leverages the proven Xorshift160 algorithm to create high-quality pseudo-random numbers.

* Rich Variety of Probability Distributions: This crate aims to implement over 40 types of probability distribution random numbers, as featured in the book ["Probability Distribution Random Number Generation Methods for Computer Simulation" by Tetsuaki Yotsuji(計算機シミュレーションのための確率分布乱数生成法/著者　四辻 哲章/プレアデス出版)](http://www.pleiades-publishing.co.jp/pdf/pdf03.html). These distributions cover a wide range of scenarios for computer simulations.

* Convenient Use of Various Structs: With a simple declaration like ```use rand_simple::StructName;```, you gain access to a variety of structs for different probability distributions, making it easy to incorporate randomness into your applications.

If you are seeking an effective solution for random number generation in Rust, ```rand_simple``` is a reliable choice. Start using it quickly and efficiently, taking advantage of its user-friendly features.

## Usage Examples
For graph-based examples, please refer to [this repository](https://github.com/Tremendous1192/demo_rand_simple).

### Uniform Distribution
```rust
let seed: u32 = rand_simple::generate_seeds!(1_usize)[0];
let mut uniform = rand_simple::Uniform::new(seed);
assert_eq!(format!("{uniform}"), "Range (Closed Interval): [0, 1]");
println!("Returns a random number -> {}", uniform.sample());

// When changing the parameters of the random variable
let min: f64 = -1_f64;
let max: f64 = 1_f64;
let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
assert_eq!(format!("{uniform}"), "Range (Closed Interval): [-1, 1]");
println!("Returns a random number -> {}", uniform.sample());
```

### Normal Distribution
```rust
let seeds: [u32; 2_usize] = rand_simple::generate_seeds!(2_usize);
let mut normal = rand_simple::Normal::new(seeds);
println!("For the initial setup, it generates random numbers following the standard normal distribution with mean μ = 0 and variance σ^2 = 1 -> {}", normal.sample());

// When changing the parameters of the random variable
let mean: f64 = -3_f64;
let variance: f64 = 2_f64;
let result: Result<(f64, f64), &str> = normal.try_set_params(mean, variance);
println!("Generating random numbers following a normal distribution with mean μ = {} and variance σ^2 = {} -> {}", mean, variance, normal.sample());
```

## Implementation Status
### Continuous distribution
* [x] Uniform distribution
* [x] 3.1 Normal distribution
* [x] 3.2 Half Normal distribution
* [x] 3.3 Log-Normal distribution
* [x] 3.4 Cauchy distribution
  * [x] Half-Cauchy distribution
* [x] 3.5 Lévy distribution
* [x] 3.6 Exponential distribution
* [x] 3.7 Laplace distribution
  * [x] Log-Laplace distribution
* [x] 3.8 Rayleigh distribution
* [x] 3.9 Weibull distribution
  * [x] Reflected Weibull distribution
  * [x] Fréchet distribution
* [x] 3.10 Gumbel distribution
* [x] 3.11 Gamma distribution
* [x] 3.12 Beta distribution
* [ ] 3.13 Dirichlet distribution
* [x] 3.14 Power Function distribution
* [ ] 3.15 Exponential Power distribution
* [x] 3.16 Erlang distribution
* [x] 3.17 Chi-Square distribution
* [x] 3.18 Chi distribution
* [x] 3.19 F distribution
* [x] 3.20 t distribution
* [x] 3.21 Inverse Gaussian distribution
* [x] 3.22 Triangular distribution
* [ ] 3.23 Pareto distribution
* [ ] 3.24 Logistic distribution
* [ ] 3.25 Hyperbolic Secant distribution
* [ ] 3.26 Raised Cosine distribution
* [ ] 3.27 Arcsine distribution
* [ ] 3.28 von Mises distribution
* [ ] 3.29 Non-Central Gammma distribution
* [ ] 3.30 Non-Central Beta distribution
* [ ] 3.31 Non-Central Chi-Square distribution
* [ ] 3.32 Non-Central Chi distribution
* [ ] 3.33 Non-Central F distribution
* [ ] 3.34 Non-Central t distribution
* [ ] 3.35 Planck distribution
### Discrete distributions
* [x] Bernoulli distribution
* [ ] 4.1 Binomial distribution
* [x] 4.2 Geometric distribution
* [ ] 4.3 Poisson distribution
* [ ] 4.4 Hypergeometric distribution
* [ ] 4.5 Multinomial distribution
* [ ] 4.6 Negative Binomial distribution
* [ ] 4.7 Negative Hypergeometric distribution
* [ ] 4.8 Logarithmic Series distribution
* [ ] 4.9 Yule-Simon distribution
* [ ] 4.10 Zipf-Mandelbrot distribution
* [ ] 4.11 Zeta distribution