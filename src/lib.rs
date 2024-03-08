pub mod algo;
pub mod demo_structs;
pub mod structs;

use crate::structs::{
    CostAllocation, CostAllocationMethods, DepreciationMethods, FinancialYear, FixedAsset,
};
use chrono::NaiveDate;

pub fn foo() {
    let t = FixedAsset {
        id: String::from("100"),
        name: String::from("car"),
        book_value: 1_000_000,
        useful_life: 2,
        salvage_value: 200_000,
        cum_depreciation: 0,
        depreciation: 0,
        depreciation_method: DepreciationMethods::StraightLine,
        cost_allocation: Some(vec![
            CostAllocation {
                method: CostAllocationMethods::Fixed,
                ratio: 0.2,
            },
            CostAllocation {
                method: CostAllocationMethods::Fixed,
                ratio: 0.5,
            },
        ]),
    };
    let fy = FinancialYear {
        date: NaiveDate::from_ymd_opt(2023, 10, 10).unwrap(),
        length: 5,
    };
    println!("{:?}", fy.dates())
}
