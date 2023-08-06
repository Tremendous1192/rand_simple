use plotters::prelude::*;

const FILE_NAME: &str = "examples/half_cauchy.png";
const CAPTION: &str = "Half Cauchy distribution";

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
            (0_f64..30_f64).step(0.1_f64).use_round().into_segmented(),
            0u32..700u32,
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
    let mut generator = rand_simple::HalfCauchy::new(1192_u32);

    // 標準分布
    println!("Initial state\n{}\n", generator);
    let mut vec = Vec::<f64>::new();
    for _ in 0..QUANTITY {
        vec.push(generator.sample());
    }
    let data: [f64; QUANTITY] = vec.try_into().unwrap();
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.3).filled())
                .margin(0)
                .data(data.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap()
        .label("Standard distribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.3)));

    // パラメータ変更
    let scale: f64 = 1.5_f64;
    let _: Result<f64, &str> = generator.try_set_params(scale);
    println!("Parameter change\n{}", generator);
    let mut vec = Vec::<f64>::new();
    for _ in 0..QUANTITY {
        vec.push(generator.sample());
    }
    let data: [f64; QUANTITY] = vec.try_into().unwrap();
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.3).filled())
                .margin(0)
                .data(data.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap()
        .label("Parameter change")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.3)));

    // 凡例の描画
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    Ok(())
}
