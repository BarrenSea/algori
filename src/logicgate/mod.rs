///逻辑门
mod logicgate;



pub use self::logicgate::NAND as NAND;
pub use self::logicgate::LogicGate as LogicGate;
pub use self::logicgate::NOT as NOT;
pub use self::logicgate::OR as OR;
pub use self::logicgate::NOR as NOR;
pub use self::logicgate::AND as AND;
pub use self::logicgate::HighLevel as HighLevel;
pub use self::logicgate::LowLevel as LowLevel;
pub use self::logicgate::XOR as XOR;
pub use self::logicgate::ThreeOR as ThreeOR;
pub use self::logicgate::ThreeAND as ThreeAND;
pub use self::logicgate::XNOR as XNOR;
pub use self::logicgate::DelayLine as DelayLine;
pub use self::logicgate::HalfAdder as HalfAdder;
pub use self::logicgate::FullAdder as FullAdder;
pub use self::logicgate::Switch as Switch;
pub use self::logicgate::EightBitSplitter as EightBitSplitter;
pub use self::logicgate::EightBitMux as EightBitMux;
pub use self::logicgate::EightBitAdder as EightBitAdder;
pub use self::logicgate::EightBitNOT as EightBitNOT;
pub use self::logicgate::EightBitOR as EightBitOR;
pub use self::logicgate::EightBitAND as EightBitAND;
pub use self::logicgate::EightBitNAND as EightBitNAND;
pub use self::logicgate::EightBitNOR as EightBitNOR;
pub use self::logicgate::EightSwitch as EightSwitch;
pub use self::logicgate::DataSelector as DataSelector;
pub use self::logicgate::ThreeDecoder as ThreeDecoder;
pub use self::logicgate::EightBitSubber as EightBitSubber;
pub use self::logicgate::EightBitNEG as EightBitNEG;
pub use self::logicgate::EightLowLevel as EightLowLevel;
pub use self::logicgate::EightHighLevel as EightHighLevel;
pub use self::logicgate::EightBitALU as EightBitALU;
