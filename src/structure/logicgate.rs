

///逻辑门
pub trait LogicGate {
    fn get_result(&self) -> Option<bool>;
}



///与非门 全为true则为false
/// # Examples
/// ```
/// use algori::structure::Nand;
/// use algori::structure::LogicGate;
/// let a = Nand{input1: Some(true), input2: Some(false)};
/// assert_eq!(a.get_result(),Some(true))
/// ```
#[derive(PartialEq)]
pub struct Nand {
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}


impl LogicGate for Nand {

    fn get_result(&self) -> Option<bool> {
	if self.input1 == Some(true){
	    if self.input2 == Some(true) {
		return Some(false);
	    }
	}
	Some(true)
    }
}

///非门 反转真值表
///非门由一个输入一致的与非门组成
/// # Examples
/// ```
/// use algori::structure::Not;
/// use algori::structure::LogicGate;
/// let a = Not{input: Some(true)};
/// let b = Not{input: Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct Not {
    pub input: Option<bool>,
}


impl LogicGate for Not {
    fn get_result(&self) -> Option<bool> {
	let a = Nand{input1: self.input, input2: self.input};
	a.get_result()
    }
}
///或门 有true则为true
///或门由三个与非门组成
/// # Examples
/// ```
/// use algori::structure::Or;
/// use algori::structure::LogicGate;
/// let a = Or{input1: Some(true),input2: Some(false)};
/// let b = Or{input1: Some(false),input2: Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(false));
/// ```
pub struct Or {
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}

impl LogicGate for Or {
    fn get_result(&self) -> Option<bool> {
	let a = Nand{input1: self.input1, input2: self.input1};
	let b = Nand{input1: self.input2, input2: self.input2};
	let c = Nand{input1: a.get_result(),input2: b.get_result()};
	c.get_result()
    }
}

///或非门 有true则为Some(false)
///或非门由四个与非门组成
/// # Examples
/// ```
/// use algori::structure::Nor;
/// use algori::structure::LogicGate;
/// let a = Nor{input1: Some(true),input2: Some(false)};
/// let b = Nor{input1: Some(false),input2: Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct Nor{
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}

impl LogicGate for Nor {
    fn get_result(&self) -> Option<bool> {
	let a = Nand{input1: self.input1, input2: self.input1};
	let b = Nand{input1: self.input2, input2: self.input2};
	let c = Nand{input1: a.get_result(),input2: b.get_result()};
	let d = Nand{input1: c.get_result(),input2: c.get_result()};
	d.get_result()
    }
}
///与门 全为true则为true
///与门由两个与非门组成
/// # Examples
/// ```
/// use algori::structure::And;
/// use algori::structure::LogicGate;
/// let a = And{input1: Some(true),input2: Some(false)};
/// let b = And{input1: Some(true),input2: Some(true)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct And {
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}

impl LogicGate for And {
    fn get_result(&self) -> Option<bool> {
	let a = Nand{input1: self.input1, input2: self.input2};
	let b = Nand{input1: a.get_result(), input2: a.get_result()};
	b.get_result()
    }
}
///高电平 输出true
/// # Examples
/// ```
/// use algori::structure::HighLevel;
/// use algori::structure::LogicGate;
/// let a = HighLevel{};
/// assert_eq!(a.get_result(),Some(true));
/// ```
pub struct HighLevel{
}
impl LogicGate for HighLevel {
    fn get_result(&self) -> Option<bool> {
	Some(true)
    }
}

///低电平 输出Some(false)
/// # Examples
/// ```
/// use algori::structure::LowLevel;
/// use algori::structure::LogicGate;
/// let a = LowLevel{};
/// assert_eq!(a.get_result(),Some(false));
/// ```
pub struct LowLevel{
}
impl LogicGate for LowLevel {
    fn get_result(&self) -> Option<bool> {
	Some(false)
    }
}

///异或门 输入不一样true
/// # Examples
/// ```
/// use algori::structure::Xor;
/// use algori::structure::LogicGate;
/// let a = Xor{input1:Some(true),input2:Some(false)};
/// let b = Xor{input1:Some(true),input2:Some(true)};
/// let c = Xor{input1:Some(false),input2:Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(false));
/// assert_eq!(c.get_result(),Some(false));
/// ```
pub struct Xor{
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}
impl LogicGate for Xor{
    fn get_result(&self) ->Option<bool> {
	let a:And = And{input1:self.input1,input2:self.input2};
	let b:Nor = Nor{input1:self.input1,input2:self.input2};
	let c:Nor = Nor{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}


///三路或门
/// # Examples
/// ```
/// use algori::structure::ThreeOr;
/// use algori::structure::LogicGate;
/// let a = ThreeOr{input1:Some(true),input2:Some(false),input3:Some(false)};
/// let b = ThreeOr{input1:Some(false),input2:Some(true),input3:Some(false)};
/// let c = ThreeOr{input1:Some(false),input2:Some(false),input3:Some(true)};
/// let d = ThreeOr{input1:Some(false),input2:Some(false),input3:Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(true));
/// assert_eq!(c.get_result(),Some(true));
/// assert_eq!(d.get_result(),Some(false));
/// ```
pub struct ThreeOr{
    pub input1: Option<bool>,
    pub input2: Option<bool>,
    pub input3: Option<bool>,
}

impl LogicGate for ThreeOr{
    fn get_result(&self) ->Option<bool> {
	let a:Or = Or{input1:self.input1,input2:self.input2};
	let b:Or = Or{input1:self.input2,input2:self.input3};
	let c:Or = Or{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}


///三路与门
/// # Examples
/// ```
/// use algori::structure::ThreeAnd;
/// use algori::structure::LogicGate;
/// let a = ThreeAnd{input1:Some(true),input2:Some(false),input3:Some(false)};
/// let b = ThreeAnd{input1:Some(false),input2:Some(true),input3:Some(false)};
/// let c = ThreeAnd{input1:Some(false),input2:Some(false),input3:Some(true)};
/// let d = ThreeAnd{input1:Some(false),input2:Some(false),input3:Some(false)};
/// let e = ThreeAnd{input1:Some(true),input2:Some(true),input3:Some(true)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(false));
/// assert_eq!(c.get_result(),Some(false));
/// assert_eq!(d.get_result(),Some(false));
/// assert_eq!(e.get_result(),Some(true));
/// ```
pub struct ThreeAnd{
    pub input1: Option<bool>,
    pub input2: Option<bool>,
    pub input3: Option<bool>,
}

impl LogicGate for ThreeAnd{
    fn get_result(&self) ->Option<bool> {
	let a:And = And{input1:self.input1,input2:self.input2};
	let b:And = And{input1:self.input2,input2:self.input3};
	let c:And = And{input1:a.get_result(),input2:b.get_result()};
	c.get_result()
    }
}
///同或门
///相同则为Some(true)
/// # Examples
/// ```
/// use algori::structure::Xnor;
/// use algori::structure::LogicGate;
/// let a = Xnor{input1:Some(true),input2:Some(false)};
/// let b = Xnor{input1:Some(true),input2:Some(true)};
/// let c = Xnor{input1:Some(false),input2:Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// assert_eq!(c.get_result(),Some(true));
/// ```
pub struct Xnor{
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}

impl LogicGate for Xnor{
    fn get_result(&self) ->Option<bool> {
	let a:Xor = Xor{input1:self.input1,input2:self.input2};
	let b:Not = Not{input: a.get_result()};
	b.get_result()
    }
}

use std::time::Duration;
///延迟线
///# Examples
///```
/// use algori::structure::DelayLine;
/// let a = DelayLine{delay: 20,input: Some(true)};
/// 
/// a.get_result();
///```

pub struct DelayLine {
    /// 延迟时间，单位为毫秒
    pub delay: u64,
    pub input: Option<bool>,
}

impl DelayLine {
    pub fn get_result(&self) -> Option<bool> {
        std::thread::sleep(Duration::from_millis(self.delay));
        self.input
    }
}

///半加器
///输出一个包含低位结果与进位的元组
/// # Examples
///```
///use algori::structure::HalfAdder;
///let a:HalfAdder = HalfAdder{input1:Some(true),input2:Some(false)};
///assert_eq!(a.get_result(),(Some(true),Some(false)));
///let a:HalfAdder = HalfAdder{input1:Some(true),input2:Some(true)};
///assert_eq!(a.get_result(),(Some(false),Some(true)));
///```
pub struct HalfAdder {
    pub input1: Option<bool>,
    pub input2: Option<bool>,
}



impl HalfAdder{
    ///返回(sum,carry)
    pub fn get_result(&self) -> (Option<bool>,Option<bool>) {
	let a:Xor = Xor{input1: self.input1,input2:self.input2};
	let b:And = And{input1: self.input1,input2:self.input2};
	(a.get_result(),b.get_result())
    }
}

///全加器
/// # Examples
///```
///use algori::structure::FullAdder;
///let a:FullAdder = FullAdder{input1:Some(true),input2:Some(false),input3:Some(false)};
///assert_eq!(a.get_result(),(Some(true),Some(false)));
///let a:FullAdder = FullAdder{input1:Some(true),input2:Some(true),input3:Some(false)};
///assert_eq!(a.get_result(),(Some(false),Some(true)));
///let a:FullAdder = FullAdder{input1:Some(true),input2:Some(true),input3:Some(true)};
///assert_eq!(a.get_result(),(Some(true),Some(true)));
///```
pub struct FullAdder {
    pub input1: Option<bool>,
    pub input2: Option<bool>,
    pub input3: Option<bool>,
}

impl FullAdder {
    ///返回(sum,carry)
    pub fn get_result(&self) ->(Option<bool>,Option<bool>) {
	let a: Xor = Xor{input1: self.input1, input2: self.input2};
	let b: And = And{input1: self.input1, input2: self.input2};
	let c: Xor = Xor{input1: a.get_result(), input2: self.input3};
	let d: And = And{input1: c.get_result(), input2: self.input3};
	let e: Or = Or{input1: b.get_result(),input2: d.get_result()};
	(c.get_result(),e.get_result())
    }
}
///一位开关
/// # Examples
///```
///use algori::structure::Switch;
///use crate::algori::structure::LogicGate;
/// let a:Switch = Switch{switch: Some(true),input:Some(false)};
///assert_eq!(a.get_result(),Some(false));
///```
pub struct Switch {
    pub switch: Option<bool>,
    pub input: Option<bool>,
}

impl LogicGate for Switch {
    fn get_result(&self) -> Option<bool> {
	if self.switch == Some(true) {
	    return self.input;
	}
	None
    }
}

///8位
pub struct EightBit{
    pub bit_1: Option<bool>,
    pub bit_2: Option<bool>,
    pub bit_3: Option<bool>,
    pub bit_4: Option<bool>,
    pub bit_5: Option<bool>,
    pub bit_6: Option<bool>,
    pub bit_7: Option<bool>,
    pub bit_8: Option<bool>,
}
///8位分线器
/// # Examples
///```
///use algori::structure::EightBitSplitter;
///let a:EightBitSplitter = EightBitSplitter{input: 201};
///let a = a.get_result();
///assert_eq!(a,(true,false,false,true,false,false,true,true));

pub struct EightBitSplitter{
    pub input: i32,
}

impl EightBitSplitter {
    pub fn get_result(&self) ->(bool,bool,bool,bool,bool,bool,bool,bool) {
	let bit1 = (self.input & 1) != 0;
        let bit2 = (self.input & 2) != 0;
        let bit3 = (self.input & 4) != 0;
        let bit4 = (self.input & 8) != 0;
        let bit5 = (self.input & 16) != 0;
        let bit6 = (self.input & 32) != 0;
        let bit7 = (self.input & 64) != 0;
        let bit8 = (self.input & 128) != 0;

        (bit1, bit2, bit3, bit4, bit5, bit6, bit7,bit8)
    }
}


///8位集线器
/// # Examples
/// ```
/// use algori::structure::EightBitMux;
/// let a = EightBitMux{input1:true,input2:false,input3:false,input4:true,input5:false,input6:false,input7:true,input8:true};
/// assert_eq!(a.get_result(),201);
/// ```
pub struct EightBitMux {
    pub input1: bool,
    pub input2: bool,
    pub input3: bool,
    pub input4: bool,
    pub input5: bool,
    pub input6: bool,
    pub input7: bool,
    pub input8: bool,
}

impl EightBitMux {
    pub fn get_result(&self) -> i32 {
        let result = (self.input8 as i32) << 7 |
                     (self.input7 as i32) << 6 |
                     (self.input6 as i32) << 5 |
                     (self.input5 as i32) << 4 |
                     (self.input4 as i32) << 3 |
                     (self.input3 as i32) << 2 |
                     (self.input2 as i32) << 1 |
                     (self.input1 as i32);
        result
    }
}
