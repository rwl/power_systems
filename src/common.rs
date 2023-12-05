use num_complex::Complex64;
use serde::{Deserialize, Serialize};

pub type Symbol = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum GeneratorCostModels {
    PiecewiseLinear = 1,
    Polynomial = 2,
}

impl Default for GeneratorCostModels {
    fn default() -> Self {
        GeneratorCostModels::Polynomial
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum AngleUnits {
    Degrees = 1,
    Radians = 2,
}

impl Default for AngleUnits {
    fn default() -> Self {
        AngleUnits::Degrees
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum BusTypes {
    Isolated = 1,
    PQ = 2,
    PV = 3,
    Ref = 4,
    Slack = 5,
}

impl Default for BusTypes {
    fn default() -> Self {
        BusTypes::PQ
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum LoadModels {
    ConstantImpedance = 1, // Z
    ConstantCurrent = 2,   // I
    ConstantPower = 3,     // P
}

impl Default for LoadModels {
    fn default() -> Self {
        LoadModels::ConstantImpedance
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum PrimeMovers {
    BA = 1,   // Energy Storage, Battery
    BT = 2,   // Turbines Used in a Binary Cycle (including those used for geothermal applications)
    CA = 3,   // Combined-Cycle – Steam Part
    CC = 4,   // Combined-Cycle - Aggregated Plant *augmentation of EIA
    CE = 5,   // Energy Storage, Compressed Air
    CP = 6,   // Energy Storage, Concentrated Solar Power
    CS = 7, // Combined-Cycle Single-Shaft Combustion turbine and steam turbine share a single generator
    CT = 8, // Combined-Cycle Combustion Turbine Part
    ES = 9, // Energy Storage, Other (Specify on Schedule 9, Comments)
    FC = 10, // Fuel Cell
    FW = 11, // Energy Storage, Flywheel
    GT = 12, // Combustion (Gas) Turbine (including jet engine design)
    HA = 13, // Hydrokinetic, Axial Flow Turbine
    HB = 14, // Hydrokinetic, Wave Buoy
    HK = 15, // Hydrokinetic, Other
    HY = 16, // Hydraulic Turbine (including turbines associated with delivery of water by pipeline)
    IC = 17, // Internal Combustion (diesel, piston, reciprocating) Engine
    PS = 18, // Energy Storage, Reversible Hydraulic Turbine (Pumped Storage)
    OT = 19, // Other – Specify on SCHEDULE 9.
    ST = 20, // Steam Turbine (including nuclear, geothermal and solar steam; does not include combined-cycle turbine)
    PVe = 21, // Photovoltaic *renaming from EIA PV to PVe to avoid conflict with BusType.PV
    WT = 22, // Wind Turbine, Onshore
    WS = 23, // Wind Turbine, Offshore
}

impl Default for PrimeMovers {
    fn default() -> Self {
        PrimeMovers::OT
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThermalFuels {
    Coal = 1,              // COL    # Anthracite Coal and Bituminous Coal
    WasteCoal = 2, // WOC    # Waste/Other Coal (includes anthracite culm, gob, fine coal, lignite waste, waste coal)
    DistillateFuelOil = 3, // DFO # Distillate Fuel Oil (Diesel, No. 1, No. 2, and No. 4
    WasteOil = 4,  // WOO    # Waste Oil Kerosene and JetFuel Butane, Propane,
    PetroleumCoke = 5, // PC  # Petroleum Coke
    ResidualFuelOil = 6, // RFO # Residual Fuel Oil (No. 5, No. 6 Fuel Oils, and Bunker Oil)
    NatualGas = 7, // NG    # Natural Gas
    OtherGas = 8,  // OOG    # Other Gas and blast furnace gas
    Nuclear = 9,   // NUC # Nuclear Fission (Uranium, Plutonium, Thorium)
    AgBiproduct = 10, // ORW    # Agricultural Crop Byproducts/Straw/Energy Crops
    MunicipalWaste = 11, // MLG    # Municipal Solid Waste – Biogenic component
    WoodWaste = 12, // WWW     # Wood Waste Liquids excluding Black Liquor (BLQ) (Includes red liquor, sludge wood, spent sulfite liquor, and other wood-based liquids)
    Geothermal = 13, // GEO     # Geothermal
    Other = 14,     // OTH     # Other
}

impl Default for ThermalFuels {
    fn default() -> Self {
        ThermalFuels::Other
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum StateTypes {
    Differential = 1,
    Algebraic = 2,
    Hybrid = 3,
}

impl Default for StateTypes {
    fn default() -> Self {
        StateTypes::Algebraic
    }
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct Cmplx64 {
    pub re: f64,
    pub im: f64,
}

impl Cmplx64 {
    pub fn as_complex(&self) -> Complex64 {
        Complex64::new(self.re, self.im)
    }
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct Metadata {
    pub module: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct InfrastructureSystemsInternal {
    pub uuid: UUID,
    pub units_info: Option<UnitsInfo>,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct UUID {
    pub value: String,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct UnitsInfo {
    pub base_power: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct Efficiency<T> {
    #[serde(rename = "in")]
    pub input: T,
    pub out: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct UpDown<T> {
    pub up: T,
    pub down: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct FromTo<T> {
    pub from: T,
    pub to: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct FlowLimit<T> {
    pub from_to: T,
    pub to_from: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct StartupShutdown<T> {
    pub startup: T,
    pub shutdown: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct L0L1<T> {
    pub l0: T,
    pub l1: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct HotWarmCold<T> {
    pub hot: T,
    pub warm: T,
    pub cold: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct TimeSeriesContainer {}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct TimeSeriesKey {
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct VariableCost<T> {
    cost: T,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct OperationalCost {}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Debug)]
pub struct Service {}
