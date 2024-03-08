use vars::structs::{FixedAsset, DepreciationMethods, CostAllocation, CostAllocationMethods};


#[test]
fn fixed_asset(){
    let fa = FixedAsset{
        id: String::from("100"),
        name: String::from("car"),
        book_value: 1_000_000,
        useful_life: 2,
        salvage_value: 200_000,
        cum_depreciation: 100_000,
        depreciation: 0,
        depreciation_method: DepreciationMethods::StraightLine,
        cost_allocation: Some(vec![
            CostAllocation {
                method: CostAllocationMethods::Fixed,
                ratio: 0.2,
            },
            CostAllocation {
                method: CostAllocationMethods::Fixed,
                ratio: 0.8,
            },
        ]),
    };
    assert_eq!(450_000, fa.calc_depreciation());
    assert_eq!(550_000, fa.calc_book_value());
    assert_eq!(550_000, fa.calc_cum_depreciation());
}