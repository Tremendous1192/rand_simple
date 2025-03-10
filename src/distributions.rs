// 子モジュールに、各疑似乱数の詳細を記述する
// 連続型確率変数
pub mod beta; // ベータ分布
pub mod cauchy; // コーシー分布
pub mod exponential; // 指数分布
pub mod frechet; // フレシェ分布
pub mod gamma; // ガンマ分布
pub mod gunbel; // ガンベル分布
pub mod half_cauchy; // 半コーシー分布
pub mod half_normal; // 半正規分布
pub mod laplace; // ラプラス分布
pub mod levy; // レヴィ分布
pub mod log_laplace; // 対数ラプラス分布
pub mod log_normal; // 対数正規分布
pub mod normal; // 正規分布
pub mod rayleigh; // レイリー分布
pub mod reflected_weibull; // 反射ワイブル分布
pub mod uniform; // 一様分布
pub mod weibull; // ワイブル分布
                 //mod dirichlet; // ディリクレ分布
pub mod power_function; // べき関数分布
                        //mod exponential_power; // 指数べき分布
pub mod chi; // χ分布
pub mod chi_square; // χ二乗分布
pub mod erlang; // アーラン分布
pub mod f; // F分布
pub mod inverse_gaussian; // 逆ガウス分布
pub mod t; // t分布
pub mod triangular; // 三角分布
                    //mod pareto; // パレート分布
                    //mod logistic; // ロジスティック分布
                    //mod heyperbolic_secant; // 双曲線正割分布
                    //mod raised_cosine; // 余弦分布
                    //mod arcsine; // 逆正弦分布
                    //mod von_mises; // フォン・ミーゼス分布
                    //mod non_central_gamma; // 非心ガンマ分布
                    //mod non_central_beta; // 非心ベータ分布
                    //mod non_central_chi_square; // 非心ガンマ二乗分布
                    //mod non_central_chi; // 非心ガンマ分布
                    //mod non_central_f; // 非心F分布
                    //mod non_central_t; // 非心t分布
                    //mod plank; // プランク分布

// 離散型確率変数
pub mod bernoulli; // ベルヌーイ分布
                   //mod Binomial // 二項分布
pub mod geometric; // 幾何分布
                   //mod Poisson // ポアソン分布
                   //mod HeyperGeometric // 超幾何分布
                   //mod Multinominal // 多項分布
                   //mod NegativeBinomial // 負の二項分布
                   //mod NegativeHeyperGeometric // 負の超幾何分布
                   //mod LogarithmicSeries // 対数級数分布
                   //mod YuleSimon // ユール・シモン分布
                   //mod ZipfMandelbrot // ジップ・マンデルブロート分布
                   //mod Zeta // ゼータ分布
