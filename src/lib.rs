mod structs;
mod algo;

use chrono::NaiveDate;
use crate::structs::{CostAllocation, CostAllocationMethods, FixedAsset, DepreciationMethods, FinancialYear};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn foo(){
    let t = FixedAsset{
        id: String::from("100"),
        name: String::from("car"),
        book_value: 1_000_000,
        useful_life: 2,
        salvage_value: 200_000,
        cum_depreciation: 0,
        depreciation: 0,
        depreciation_method: DepreciationMethods::StraightLine,
        cost_allocation: Some(vec![CostAllocation{method:CostAllocationMethods::Fixed, ratio:0.2},CostAllocation{method:CostAllocationMethods::Fixed, ratio:0.5} ])
    };
    let fy=  FinancialYear{
        date: NaiveDate::from_ymd_opt(2023, 10, 10).unwrap(),
        length: 5
    };
    println!("{:?}", fy.dates())

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
