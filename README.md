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
// Generate a single seed value for initializing the random number generator
let seed: u32 = rand_simple::generate_seeds!(1_usize)[0];

// Create a new instance of the Uniform distribution with the generated seed
let mut uniform = rand_simple::Uniform::new(seed);

// Check the default range of the uniform distribution and print it
assert_eq!(format!("{uniform}"), "Range (Closed Interval): [0, 1]");
println!("Returns a random number -> {}", uniform.sample());

// When changing the parameters of the random variable

// Define new minimum and maximum values for the uniform distribution
let min: f64 = -1_f64;
let max: f64 = 1_f64;

// Attempt to set the new parameters for the uniform distribution
let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);

// Check the updated range of the uniform distribution and print it
assert_eq!(format!("{uniform}"), "Range (Closed Interval): [-1, 1]");
println!("Returns a random number -> {}", uniform.sample());

```

### Normal Distribution
```rust
// Generate two seed values for initializing the random number generator
let seeds: [u32; 2_usize] = rand_simple::generate_seeds!(2_usize);

// Create a new instance of the Normal distribution with the generated seeds
let mut normal = rand_simple::Normal::new(seeds);

// Check the default parameters of the normal distribution (mean = 0, std deviation = 1) and print it
assert_eq!(format!("{normal}"), "N(Mean, Std^2) = N(0, 1^2)");
println!("Returns a random number -> {}", normal.sample());

// When changing the parameters of the random variable

// Define new mean and standard deviation values for the normal distribution
let mean: f64 = -3_f64;
let std: f64 = 2_f64;

// Attempt to set the new parameters for the normal distribution
let result: Result<(f64, f64), &str> = normal.try_set_params(mean, std);

// Check the updated parameters of the normal distribution and print it
assert_eq!(format!("{normal}"), "N(Mean, Std^2) = N(-3, 2^2)");
println!("Returns a random number -> {}", normal.sample());

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
  * [ ] Half Exponential Power distribution
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