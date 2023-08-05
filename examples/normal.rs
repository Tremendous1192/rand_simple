use plotters::prelude::*;

const FILE_NAME: &str = "examples/normal.png";
const CAPTION: &str = "Normal distribution";

const QUANTITY: usize = 10_000_usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // キャンバスの生成
    let root = BitMapBackend::new(FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // キャンバスの設定
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .caption(CAPTION, ("sans-serif", 50.0))
        .build_cartesian_2d(
            (-10_f64..5_f64).step(0.1_f64).use_round().into_segmented(),
            0u32..500u32,
        )?;
    // 軸の設定
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Random variable x")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    // 乱数生成器
    let mut generator = rand_simple::Normal::new([1192u32, 765u32]);

    // 標準分布
    let mut vec = Vec::<f64>::new();
    for _ in 0..QUANTITY {
        vec.push(generator.sample());
    }
    let data: [f64; QUANTITY] = vec.try_into().unwrap();
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.3).filled())
                .margin(1)
                .data(data.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap();

    // パラメータ変更
    let mean: f64 = -3f64;
    let variance: f64 = 2f64;
    let _: Result<(f64, f64), &str> = generator.try_set_params(mean, variance);
    let mut vec = Vec::<f64>::new();
    for _ in 0..QUANTITY {
        vec.push(generator.sample());
    }
    let data: [f64; QUANTITY] = vec.try_into().unwrap();
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.5).filled())
                .margin(1)
                .data(data.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap();

    Ok(())
}

