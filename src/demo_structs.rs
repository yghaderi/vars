use crate::structs::{
    CostAllocation, CostAllocationMethods, CostCenter, CostCenterCategory, DepreciationMethods,
    FinancialYear, Firm, FirmCategory, FixedAsset, Input,BalanceSheet
};
use chrono::NaiveDate;

pub fn demo_fixed_asset() -> FixedAsset {
    FixedAsset {
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
    }
}

pub fn financial_year() -> FinancialYear {
    FinancialYear {
        date: NaiveDate::from_ymd_opt(2023, 10, 10).unwrap(),
        length: 5,
    }
}

pub fn demo_firm() -> Firm {
    let cc1 = CostCenter {
        id: String::from("10"),
        name: String::from("ستاد"),
        category: CostCenterCategory::Operational,
        input: Some(Input {
            fixed_assets: Some(vec![demo_fixed_asset(), demo_fixed_asset()]),
        }),
    };
    Firm {
        id: String::from("1"),
        name: String::from("شپدیس"),
        financial_year: financial_year(),
        category: FirmCategory::Production,
        cost_centers: vec![cc1],
    }
}

pub fn demo_balance_sheet()->BalanceSheet{
    BalanceSheet {
        property_plant_and_equipment: 1_100_000
    }
}
