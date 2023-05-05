#[test]
fn test_sample() {
    let generator = crate::Rayleigh::new(1192u32);

    assert_eq!(generator.sample(), 1.742465812716269f64);
    assert_eq!(generator.sample(), 1.029740754699521f64);
    assert_eq!(generator.sample(), 1.5551377227087484f64);
    assert_eq!(generator.sample(), 1.5541905031281156f64);
    assert_eq!(generator.sample(), 1.0430521250087208f64);

    assert_eq!(generator.sample(), 1.9825240964845519f64);
    assert_eq!(generator.sample(), 1.4443423438260148f64);
    assert_eq!(generator.sample(), 2.0637647855216175f64);
    assert_eq!(generator.sample(), 1.3523757911883567f64);
    assert_eq!(generator.sample(), 0.5942825746183323f64);
}