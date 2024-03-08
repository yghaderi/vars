use vars::demo_structs::demo_fixed_asset;

#[test]
fn fixed_asset() {
    let fa = demo_fixed_asset();
    assert_eq!(450_000, fa.calc_depreciation());
    assert_eq!(550_000, fa.calc_book_value());
    assert_eq!(550_000, fa.calc_cum_depreciation());
}

