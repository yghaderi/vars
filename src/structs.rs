use chrono::{Datelike, NaiveDate};
use std::cmp::min;
use std::vec;

// CostingMethods *******************************************************************
enum CostingMethods {
    Variable,
    Absorption,
}

impl From<CostingMethods> for String {
    fn from(state: CostingMethods) -> String {
        match state {
            CostingMethods::Variable => "variable".to_owned(),
            CostingMethods::Absorption => "absorption".to_owned(),
        }
    }
}

pub struct CostingMethod {
    method: CostingMethods,
}

// CostAllocation *******************************************************************
#[derive(Clone, Debug)]
pub enum CostAllocationMethods {
    Fixed,
    Variable,
}

impl From<CostAllocationMethods> for String {
    fn from(state: CostAllocationMethods) -> String {
        match state {
            CostAllocationMethods::Fixed => "fixed".to_owned(),
            CostAllocationMethods::Variable => "variable".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CostAllocation {
    pub method: CostAllocationMethods,
    pub ratio: f64,
}

// FixedAsset ***********************************************************************
#[derive(Clone, Debug)]
pub enum DepreciationMethods {
    StraightLine,
    DecliningBalance,
    DoubleDecliningBalance,
}
impl From<DepreciationMethods> for String {
    fn from(state: DepreciationMethods) -> String {
        match state {
            DepreciationMethods::StraightLine => "straight_line".to_owned(),
            DepreciationMethods::DecliningBalance => "declining_balance".to_owned(),
            DepreciationMethods::DoubleDecliningBalance => "double_declining_balance".to_owned(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct FixedAsset {
    pub id: String,
    pub name: String,
    pub book_value: u64,
    pub useful_life: u64,
    pub salvage_value: u64,
    pub cum_depreciation: u64,
    pub depreciation: u64,
    pub depreciation_method: DepreciationMethods,
    pub cost_allocation: Option<Vec<CostAllocation>>,
}

impl FixedAsset {
    pub fn calc_depreciation(&self) -> u64 {
        match self.depreciation_method {
            DepreciationMethods::StraightLine => {
                let depr = (self.book_value + self.cum_depreciation - self.salvage_value)
                    / self.useful_life;
                let r_useful_life = self.useful_life - (self.cum_depreciation / depr);
                return depr * min(1, r_useful_life);
            }
            DepreciationMethods::DecliningBalance => 1 + 2,
            DepreciationMethods::DoubleDecliningBalance => 1 + 2,
        }
    }
    pub fn calc_book_value(&self) -> u64 {
        self.book_value - self.calc_depreciation()
    }
    pub fn calc_cum_depreciation(&self) -> u64 {
        self.cum_depreciation + self.calc_depreciation()
    }

    pub fn gen(&self) -> FixedAsset {
        FixedAsset {
            id: String::from(&self.id),
            name: String::from(&self.name),
            book_value: self.calc_book_value(),
            useful_life: self.useful_life,
            salvage_value: self.salvage_value,
            cum_depreciation: self.calc_cum_depreciation(),
            depreciation: self.calc_depreciation(),
            depreciation_method: self.depreciation_method.clone(),
            cost_allocation: self.cost_allocation.clone(),
        }
    }
}

// ChangeFactor ***********************************************************************
struct ChangeFactor {
    year: u8,
    factor: f64,
}

struct BaseChangeFactor {
    id: String,
    name: String,
    factors: Vec<ChangeFactor>,
}

struct ExtraChange {
    id: String,
    name: String,
    factor: Vec<ChangeFactor>,
}

// NormFinancialRatio *******************************************************************
struct NormFinancialRatio {
    current: f64,
    target: f64,
    begin_improvement_year: u8,
    mature_year: u8,
}

// Inventory ****************************************************************************
enum ManagementExcessDeficit {
    Buy,
    Sell,
}

impl From<ManagementExcessDeficit> for String {
    fn from(state: ManagementExcessDeficit) -> String {
        match state {
            ManagementExcessDeficit::Buy => "buy".to_owned(),
            ManagementExcessDeficit::Sell => "sell".to_owned(),
        }
    }
}
struct ManagementApproach {
    excess: ManagementExcessDeficit,
    deficit: ManagementExcessDeficit,
}
struct Inventory {
    qty: f64,
    management_approach: ManagementApproach,
    norm_ratio: NormFinancialRatio,
}

// RawMaterial *************************************************************************
struct RawMaterial {
    id: String,
    name: String,
    unit: String,
    rate: u64,
    factor: ExtraChange,
    inventory: Inventory,
}
// Consumption ***********************************************************************
enum ConsumptionFactor {
    Capacity,
    Qty,
}
struct Consumption {
    raw_material_id: String,
    ratio: f64,
    factor: ConsumptionFactor,
}

// Product ****************************************************************************

struct Product {
    id: String,
    name: String,
    unit: String,
    rate: u64,
    factor: ExtraChange,
    inventory: Inventory,
    capacity: u64,
    qty: u64,
    consumption: Consumption,
}

// FinancialYear **********************************************************************
#[derive(Clone, Debug)]
pub struct FinancialYear {
    pub date: NaiveDate,
    pub length: u8,
}

impl FinancialYear {
    pub fn dates(&self) -> Vec<NaiveDate> {
        let mut dates: Vec<NaiveDate> = vec![];
        for i in 0..self.length {
            dates.push(
                NaiveDate::from_ymd_opt(
                    self.date.year() + i as i32,
                    self.date.month(),
                    self.date.day(),
                )
                .unwrap(),
            )
        }
        dates
    }
}
// Input ***********************************************************************
pub struct Input {
    pub fixed_assets: Option<Vec<FixedAsset>>,
}
// CostCenter **********************************************************************
pub enum CostCenterCategory {
    Product,
    Service,
    Operational,
}

impl From<CostCenterCategory> for String {
    fn from(state: CostCenterCategory) -> String {
        match state {
            CostCenterCategory::Product => "product".to_owned(),
            CostCenterCategory::Service => "service".to_owned(),
            CostCenterCategory::Operational => "operational".to_owned(),
        }
    }
}
pub struct CostCenter {
    pub id: String,
    pub name: String,
    pub category: CostCenterCategory,
    pub input: Option<Input>,
}

// Firm **************************************************************************
pub enum FirmCategory {
    Production,
}

impl From<FirmCategory> for String {
    fn from(state: FirmCategory) -> String {
        match state {
            FirmCategory::Production => "production".to_owned(),
        }
    }
}
pub struct Firm {
    pub id: String,
    pub name: String,
    pub financial_year: FinancialYear,
    pub category: FirmCategory,
    pub cost_centers: Vec<CostCenter>,
}

// FinancialStatements *********************************************************
#[derive(Debug, PartialEq)]
pub struct BalanceSheet {
    pub fixed_asset: u64,
    pub inventory: u64,
}
