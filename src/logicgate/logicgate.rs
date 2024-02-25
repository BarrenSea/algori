///逻辑门
pub trait LogicGate {
    fn get_result(&self) -> Option<bool>;
}



///与非门 全为true则为false
/// # Examples
/// ```
/// use algori::logicgate::NAND;
/// use algori::logicgate::LogicGate;
/// let a = NAND{input1: &Some(true), input2: &Some(false)};
/// assert_eq!(a.get_result(),Some(true))
/// ```
#[derive(PartialEq)]
pub struct NAND<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}


impl<'a> LogicGate for NAND<'a> {

    fn get_result(&self) -> Option<bool> {
	if *self.input1 == Some(true){
	    if *self.input2 == Some(true) {
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
/// use algori::logicgate::NOT;
/// use algori::logicgate::LogicGate;
/// let a = NOT{input: &Some(true)};
/// let b = NOT{input: &Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct NOT<'a> {
    pub input: &'a Option<bool>,
}


impl<'a> LogicGate for NOT<'a> {
    fn get_result(&self) -> Option<bool> {
	let a = NAND{input1: &self.input, input2: &self.input};
	a.get_result()
    }
}
///或门 有true则为true
///或门由三个与非门组成
/// # Examples
/// ```
/// use algori::logicgate::OR;
/// use algori::logicgate::LogicGate;
/// let a = OR{input1: &Some(true),input2: &Some(false)};
/// let b = OR{input1: &Some(false),input2: &Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(false));
/// ```
pub struct OR<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}

impl<'a> LogicGate for OR<'a> {
    fn get_result(&self) -> Option<bool> {
	let a = NAND{input1: self.input1, input2: self.input1};
	let b = NAND{input1: self.input2, input2: self.input2};
	let c = NAND{input1: &a.get_result(),input2: &b.get_result()};
	c.get_result()
    }
}

///或非门 有true则为Some(false)
///或非门由四个与非门组成
/// # Examples
/// ```
/// use algori::logicgate::NOR;
/// use algori::logicgate::LogicGate;
/// let a = NOR{input1: &Some(true),input2: &Some(false)};
/// let b = NOR{input1: &Some(false),input2: &Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct NOR<'a>{
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}

impl<'a> LogicGate for NOR<'a> {
    fn get_result(&self) -> Option<bool> {
	let a = NAND{input1: self.input1, input2: self.input1};
	let b = NAND{input1: self.input2, input2: self.input2};
	let c = NAND{input1: &a.get_result(),input2: &b.get_result()};
	let d = NAND{input1: &c.get_result(),input2: &c.get_result()};
	d.get_result()
    }
}
///与门 全为true则为true
///与门由两个与非门组成
/// # Examples
/// ```
/// use algori::logicgate::AND;
/// use algori::logicgate::LogicGate;
/// let a = AND{input1: &Some(true),input2: &Some(false)};
/// let b = AND{input1: &Some(true),input2: &Some(true)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// ```
pub struct AND<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}

impl<'a> LogicGate for AND<'a> {
    fn get_result(&self) -> Option<bool> {
	let a = NAND{input1: self.input1, input2: self.input2};
	let b = NAND{input1: &a.get_result(), input2: &a.get_result()};
	b.get_result()
    }
}
///高电平 输出true
/// # Examples
/// ```
/// use algori::logicgate::HighLevel;
/// use algori::logicgate::LogicGate;
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
/// use algori::logicgate::LowLevel;
/// use algori::logicgate::LogicGate;
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
/// use algori::logicgate::XOR;
/// use algori::logicgate::LogicGate;
/// let a = XOR{input1:& Some(true),input2:& Some(false)};
/// let b = XOR{input1:& Some(true),input2:& Some(true)};
/// let c = XOR{input1:& Some(false),input2:& Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(false));
/// assert_eq!(c.get_result(),Some(false));
/// ```
pub struct XOR<'a>{
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}
impl<'a> LogicGate for XOR<'a>{
    fn get_result(&self) ->Option<bool> {
	let a:AND = AND{input1:self.input1,input2:self.input2};
	let b:NOR = NOR{input1:self.input1,input2:self.input2};
	let c:NOR = NOR{input1:&a.get_result(),input2:&b.get_result()};
	c.get_result()
    }
}


///三路或门
/// # Examples
/// ```
/// use algori::logicgate::ThreeOR;
/// use algori::logicgate::LogicGate;
/// let a = ThreeOR{input1:& Some(true),input2:& Some(false),input3:& Some(false)};
/// let b = ThreeOR{input1:& Some(false),input2:& Some(true),input3:& Some(false)};
/// let c = ThreeOR{input1:& Some(false),input2:& Some(false),input3:& Some(true)};
/// let d = ThreeOR{input1:& Some(false),input2:& Some(false),input3:& Some(false)};
/// assert_eq!(a.get_result(),Some(true));
/// assert_eq!(b.get_result(),Some(true));
/// assert_eq!(c.get_result(),Some(true));
/// assert_eq!(d.get_result(),Some(false));
/// ```
pub struct ThreeOR<'a>{
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
    pub input3: &'a Option<bool>,
}

impl<'a> LogicGate for ThreeOR<'a>{
    fn get_result(&self) ->Option<bool> {
	let a:OR = OR{input1:self.input1,input2:self.input2};
	let b:OR = OR{input1:self.input2,input2:self.input3};
	let c:OR = OR{input1:&a.get_result(),input2:&b.get_result()};
	c.get_result()
    }
}


///三路与门
/// # Examples
/// ```
/// use algori::logicgate::ThreeAND;
/// use algori::logicgate::LogicGate;
/// let a = ThreeAND{input1:& Some(true),input2:& Some(false),input3:& Some(false)};
/// let b = ThreeAND{input1:& Some(false),input2:& Some(true),input3:& Some(false)};
/// let c = ThreeAND{input1:& Some(false),input2:& Some(false),input3:& Some(true)};
/// let d = ThreeAND{input1:& Some(false),input2:& Some(false),input3:& Some(false)};
/// let e = ThreeAND{input1:& Some(true),input2:& Some(true),input3:& Some(true)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(false));
/// assert_eq!(c.get_result(),Some(false));
/// assert_eq!(d.get_result(),Some(false));
/// assert_eq!(e.get_result(),Some(true));
/// ```
pub struct ThreeAND<'a>{
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
    pub input3: &'a Option<bool>,
}

impl<'a> LogicGate for ThreeAND<'a>{
    fn get_result(&self) ->Option<bool> {
	let a:AND = AND{input1:self.input1,input2:self.input2};
	let b:AND = AND{input1:self.input2,input2:self.input3};
	let c:AND = AND{input1:&a.get_result(),input2:&b.get_result()};
	c.get_result()
    }
}
///同或门
///相同则为Some(true)
/// # Examples
/// ```
/// use algori::logicgate::XNOR;
/// use algori::logicgate::LogicGate;
/// let a = XNOR{input1:&Some(true),input2:&Some(false)};
/// let b = XNOR{input1:&Some(true),input2:&Some(true)};
/// let c = XNOR{input1:&Some(false),input2:&Some(false)};
/// assert_eq!(a.get_result(),Some(false));
/// assert_eq!(b.get_result(),Some(true));
/// assert_eq!(c.get_result(),Some(true));
/// ```
pub struct XNOR<'a>{
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}

impl<'a> LogicGate for XNOR<'a>{
    fn get_result(&self) ->Option<bool> {
	let a:XOR = XOR{input1:self.input1,input2:self.input2};
	let b:NOT = NOT{input: &a.get_result()};
	b.get_result()
    }
}

use std::time::Duration;
/////延迟线
/////# Examples
/////```
///// use algori::logicgate::DelayLine;
///// let a = DelayLine{delay: 20,input: &Some(true)};
///// 
///// a.get_result();
/////```

// pub struct DelayLine<'a> {
//     /// 延迟时间，单位为毫秒
//     pub input: &'a[Option<bool>;2],
// }

// impl<'a> DelayLine<'a> {
//     pub fn get_result(&self) -> Option<bool> {
//         std::thread::sleep(Duration::from_millis(self.delay));
//         *self.input
//     }
// }

///半加器
///输出一个包含低位结果与进位的元组
/// # Examples
///```
///use algori::logicgate::HalfAdder;
///let a:HalfAdder = HalfAdder{input1:&Some(true),input2:&Some(false)};
///assert_eq!(a.get_result(),(Some(true),Some(false)));
///let a:HalfAdder = HalfAdder{input1:&Some(true),input2:&Some(true)};
///assert_eq!(a.get_result(),(Some(false),Some(true)));
///```
pub struct HalfAdder<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
}



impl<'a> HalfAdder<'a>{
    ///返回(sum,carry)
    pub fn get_result(&self) -> (Option<bool>,Option<bool>) {
	let a:XOR = XOR{input1: self.input1,input2:self.input2};
	let b:AND = AND{input1: self.input1,input2:self.input2};
	(a.get_result(),b.get_result())
    }
}

///全加器
/// # Examples
///```
///use algori::logicgate::FullAdder;
///let a:FullAdder = FullAdder{input1:&Some(true),input2:&Some(false),input3:&Some(false)};
///assert_eq!(a.get_result(),(Some(true),Some(false)));
///let a:FullAdder = FullAdder{input1:&Some(true),input2:&Some(true),input3:&Some(false)};
///assert_eq!(a.get_result(),(Some(false),Some(true)));
///let a:FullAdder = FullAdder{input1:&Some(true),input2:&Some(true),input3:&Some(true)};
///assert_eq!(a.get_result(),(Some(true),Some(true)));
///let a:FullAdder = FullAdder{input1:&Some(false),input2:&Some(false),input3:&Some(false)};
///assert_eq!(a.get_result(),(Some(false),Some(false)));
///```
pub struct FullAdder<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
    pub input3: &'a Option<bool>,
}

impl<'a> FullAdder<'a> {
    ///返回(sum,carry)
    pub fn get_result(&self) ->(Option<bool>,Option<bool>) {
	let a: XOR = XOR{input1: self.input1, input2: self.input2};
	let b: AND = AND{input1: self.input1, input2: self.input2};
	let c: XOR = XOR{input1: &a.get_result(), input2: self.input3};
	let d: AND = AND{input1: &a.get_result(), input2: self.input3};
	let e: OR = OR{input1: &b.get_result(),input2: &d.get_result()};
	(c.get_result(),e.get_result())
    }
}
///一位开关
/// # Examples
///```
///use algori::logicgate::Switch;
///use crate::algori::logicgate::LogicGate;
/// let a:Switch = Switch{switch: &Some(true),input:&Some(false)};
///assert_eq!(a.get_result(),Some(false));
///```
pub struct Switch<'a> {
    pub switch: &'a Option<bool>,
    pub input: &'a Option<bool>,
}

impl<'a> LogicGate for Switch<'a> {
    fn get_result(&self) -> Option<bool> {
	if *self.switch == Some(true) {
	    return *self.input;
	}
	None
    }
}

///八位开关
/// # Examples
///```
///use algori::logicgate::Switch;
///use crate::algori::logicgate::LogicGate;
/// let a:Switch = Switch{switch: &Some(true),input:&Some(false)};
///assert_eq!(a.get_result(),Some(false));
///```
pub struct EightSwitch<'a> {
    pub switch: &'a Option<bool>,
    pub input: i8,
}

impl<'a>  EightSwitch<'a> {
    fn get_result(&self) -> Option<i8> {
	if *self.switch == Some(true) {
	    return Some(self.input);
	}
	None
    }
}

///八位分线器
/// # Examples
///```
///use algori::logicgate::EightBitSplitter;
///let a:EightBitSplitter = EightBitSplitter{input: -55};
///let a = a.get_result();
///assert_eq!(a,(Some(true),Some(false),Some(false),Some(true),Some(false),Some(false),Some(true),Some(true)));
/// let a:EightBitSplitter = EightBitSplitter{input: 33};
/// let a = a.get_result();
/// assert_eq!(a,(Some(true),Some(false),Some(false),Some(false),Some(false),Some(true),Some(false),Some(false)));
///```
pub struct EightBitSplitter{
    pub input: i8,
}

impl EightBitSplitter {
    pub fn get_result(&self) ->(Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>) {
	let bit1 = (self.input & 1) != 0;
        let bit2 = (self.input & 2) != 0;
        let bit3 = (self.input & 4) != 0;
        let bit4 = (self.input & 8) != 0;
        let bit5 = (self.input & 16) != 0;
        let bit6 = (self.input & 32) != 0;
        let bit7 = (self.input & 64) != 0;
        let bit8 = (self.input & -128) != 0;

        (Some(bit1), Some(bit2), Some(bit3), Some(bit4), Some(bit5), Some(bit6), Some(bit7),Some(bit8))
    }
}


///八位集线器
/// # Examples
/// ```
/// use algori::logicgate::EightBitMux;
/// let a = EightBitMux{input1:& Some(true),input2:& Some(false),input3:& Some(false),input4:& Some(true),input5:& Some(false),input6:& Some(false),input7:& Some(true),input8:& Some(true)};
/// assert_eq!(a.get_result(),-55);
/// let a = EightBitMux{input1:& Some(false),input2: & Some(false),input3: & Some(true),input4: & Some(true),input5: & Some(false),input6: & Some(true),input7: & Some(true),input8: & Some(false)};
/// assert_eq!(a.get_result(),108);
/// ```
pub struct EightBitMux<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
    pub input3: &'a Option<bool>,
    pub input4: &'a Option<bool>,
    pub input5: &'a Option<bool>,
    pub input6: &'a Option<bool>,
    pub input7: &'a Option<bool>,
    pub input8: &'a Option<bool>,
}

impl<'a> EightBitMux<'a> {
    pub fn get_result(&self) -> i8 {
        let result = (self.input8.unwrap_or(false) as i8) << 7 |
                     (self.input7.unwrap_or(false) as i8) << 6 |
                     (self.input6.unwrap_or(false) as i8) << 5 |
                     (self.input5.unwrap_or(false) as i8) << 4 |
                     (self.input4.unwrap_or(false) as i8) << 3 |
                     (self.input3.unwrap_or(false) as i8) << 2 |
                     (self.input2.unwrap_or(false) as i8) << 1 |
                     (self.input1.unwrap_or(false) as i8);
        result
    }
}

///八位加法器
/// # get 1 bool input and 2 i8 inputs
/// ## return 1 EightBit Output and Carry bool
/// # Examples
/// ```
///    use algori::logicgate::EightBitAdder;
///         let adder = EightBitAdder {
///             input1: &Some(false),
///             input2: 1_i8,
///             input3: 1_i8,
///         };
///         let result = adder.get_result();
///
///         // 预期的加法结果
///         let expected_output = 2;
///
///         // 预期的进位
///         let expected_carry = Some(false);
///
///         // 检查加法器的输出
///         assert_eq!(result, (expected_output, expected_carry));
/// ```
pub struct EightBitAdder<'a>{
    pub input1: &'a Option<bool>,
    pub input2: i8,
    pub input3: i8,
}

impl<'a> EightBitAdder<'a> {
    /// 返回 (低八位结果, 进位)

    pub fn get_result(&self) -> (i8, Option<bool>) {
        // 分割输入
        let splitter_one = EightBitSplitter { input: self.input2 }.get_result();
        let splitter_two = EightBitSplitter { input: self.input3 }.get_result();
        // 逐个全加器进行相加
        let adder_one = FullAdder { input1: self.input1, input2: & splitter_one.0, input3: & splitter_two.0 }.get_result();
        let adder_two = FullAdder { input1: & adder_one.1, input2: & splitter_one.1, input3: & splitter_two.1 }.get_result();
        let adder_three = FullAdder { input1: & adder_two.1, input2: & splitter_one.2, input3: & splitter_two.2 }.get_result();
        let adder_four = FullAdder { input1: & adder_three.1, input2: & splitter_one.3, input3: & splitter_two.3 }.get_result();
        let adder_five = FullAdder { input1: & adder_four.1, input2: & splitter_one.4, input3: & splitter_two.4 }.get_result();
        let adder_six = FullAdder { input1: & adder_five.1, input2: & splitter_one.5, input3: & splitter_two.5 }.get_result();
        let adder_seven = FullAdder { input1: & adder_six.1, input2: & splitter_one.6, input3: & splitter_two.6 }.get_result();
        let adder_eight = FullAdder { input1: & adder_seven.1, input2: & splitter_one.7, input3: & splitter_two.7 }.get_result();
        // 合并全加器的结果
        let selection = EightBitMux{input1:& adder_one.0, input2:& adder_two.0, input3:& adder_three.0, input4:& adder_four.0, input5:& adder_five.0, input6:& adder_six.0, input7:& adder_seven.0, input8:& adder_eight.0 }.get_result();

        // 返回最终结果及进位
        (selection, adder_eight.1)
    }
}


/// 八位高电平 输出1
/// # Examples
/// ```
/// use algori::logicgate::EightHighLevel;
/// let a = EightHighLevel{};
/// assert_eq!(a.get_result(),1);
/// ```
pub struct EightHighLevel{
}
impl  EightHighLevel {
    pub fn get_result(&self) -> i8 {
	1
    }
}

/// 八位低电平 输出0
/// # Examples
/// ```
/// use algori::logicgate::EightLowLevel;
/// let a = EightLowLevel{};
/// assert_eq!(a.get_result(),0);
/// ```
pub struct EightLowLevel{
}

impl  EightLowLevel {
    pub fn get_result(&self) -> i8 {
	0
    }
}


///八位取反器
/// # 输出相反数
/// # Examples
/// ```
/// use algori::logicgate::EightBitNEG;
/// let b = EightBitNEG{input: 10}.get_result();
/// assert_eq!(b, -10);
///
/// ```
pub struct EightBitNEG{
    pub input: i8,
}
impl EightBitNEG{
    pub fn get_result(&self) ->i8 {
	let a = EightBitNOT{input:self.input}.get_result();
	let b = EightHighLevel{}.get_result();
	EightBitAdder{input1:&None,input2: b, input3: a}.get_result().0
    }
}

///八位减法器
/// # Examples
/// ```
/// use algori::logicgate::EightBitSubber;
/// let a = EightBitSubber{input1:19,input2:20}.get_result();
/// assert_eq!(a,-1);
/// let a = EightBitSubber{input1:39,input2:20}.get_result();
/// assert_eq!(a,19);
/// let q = EightBitSubber{input1: -20,input2: -100}.get_result();
/// assert_eq!(q,80);
/// ```
pub struct EightBitSubber{
    pub input1: i8,
    pub input2: i8,
}
impl EightBitSubber{
    pub fn get_result(&self) -> i8 {
	let b = EightBitNEG{input: self.input1 }.get_result();
	EightBitNEG{input: EightBitAdder{input1: &None,input2: b,input3: self.input2}.get_result().0}.get_result()
    }
}

///八位非
/// # Examples
/// ```
///    use algori::logicgate::EightBitNOT;
///    let a = EightBitNOT{input: 80}.get_result();
///    assert_eq!(a, -81);
/// ```
pub struct EightBitNOT{
    pub input: i8,
}

impl EightBitNOT {
    pub fn get_result(&self) -> i8 {
	let a = EightBitSplitter{input: self.input}.get_result();
	let b = NOT{input:& a.0}.get_result();
	let c = NOT{input:& a.1}.get_result();
	let d = NOT{input:& a.2}.get_result();
	let e = NOT{input:& a.3}.get_result();
	let f = NOT{input:& a.4}.get_result();
	let g = NOT{input:& a.5}.get_result();
	let h = NOT{input:& a.6}.get_result();
	let i = NOT{input:& a.7}.get_result();
	EightBitMux{input1:& b,input2:& c,input3:& d,input4:& e,input5:& f,input6:& g,input7:& h,input8:& i}.get_result()
    }
}

///八位与非
/// # Examples
/// ```
///    use algori::logicgate::EightBitNAND;
///    let a = EightBitNAND{input1: 80,input2:21}.get_result();
///    assert_eq!(a,-17);
/// ```
pub struct EightBitNAND{
    pub input1: i8,
    pub input2: i8,
}

impl EightBitNAND {
    pub fn get_result(&self) -> i8 {
	let a = EightBitSplitter{input: self.input1}.get_result();
	let b = EightBitSplitter{input: self.input2}.get_result();
	let c = NAND{input1:& a.0,input2:& b.0}.get_result();
	let d = NAND{input1:& a.1,input2:& b.1}.get_result();
	let e = NAND{input1:& a.2,input2:& b.2}.get_result();
	let f = NAND{input1:& a.3,input2:& b.3}.get_result();
	let g = NAND{input1:& a.4,input2:& b.4}.get_result();
	let h = NAND{input1:& a.5,input2:& b.5}.get_result();
	let i = NAND{input1:& a.6,input2:& b.6}.get_result();
	let j = NAND{input1:& a.7,input2:& b.7}.get_result();
	EightBitMux{input1:& c,input2: & d,input3:& e,input4:& f,input5:& g,input6:& h,input7:& i,input8:& j}.get_result()
    }
}
///八位与
/// # Examples
/// ```
///    use algori::logicgate::EightBitAND;
///    let a = EightBitAND{input1: 80,input2:21}.get_result();
///    assert_eq!(a,16);
/// ```
pub struct EightBitAND{
    pub input1: i8,
    pub input2: i8,
}

impl EightBitAND {
    pub fn get_result(&self) -> i8 {
	let a = EightBitSplitter{input: self.input1}.get_result();
	let b = EightBitSplitter{input: self.input2}.get_result();
	let c = AND{input1:& a.0,input2:& b.0}.get_result();
	let d = AND{input1:& a.1,input2:& b.1}.get_result();
	let e = AND{input1:& a.2,input2:& b.2}.get_result();
	let f = AND{input1:& a.3,input2:& b.3}.get_result();
	let g = AND{input1:& a.4,input2:& b.4}.get_result();
	let h = AND{input1:& a.5,input2:& b.5}.get_result();
	let i = AND{input1:& a.6,input2:& b.6}.get_result();
	let j = AND{input1:& a.7,input2:& b.7}.get_result();
	EightBitMux{input1:& c,input2: & d,input3:& e,input4:& f,input5:& g,input6:& h,input7:& i,input8:& j}.get_result()
    }
}




///八位或非
/// # Examples
/// ```
///    use algori::logicgate::EightBitNOR;
///    let a = EightBitNOR{input1: 80,input2:21}.get_result();
///    assert_eq!(a,-17);
/// ```
pub struct EightBitNOR{
    pub input1: i8,
    pub input2: i8,
}

impl EightBitNOR {
    pub fn get_result(&self) -> i8 {
	let a = EightBitSplitter{input: self.input1}.get_result();
	let b = EightBitSplitter{input: self.input2}.get_result();
	let c = NAND{input1:& a.0,input2:& b.0}.get_result();
	let d = NAND{input1:& a.1,input2:& b.1}.get_result();
	let e = NAND{input1:& a.2,input2:& b.2}.get_result();
	let f = NAND{input1:& a.3,input2:& b.3}.get_result();
	let g = NAND{input1:& a.4,input2:& b.4}.get_result();
	let h = NAND{input1:& a.5,input2:& b.5}.get_result();
	let i = NAND{input1:& a.6,input2:& b.6}.get_result();
	let j = NAND{input1:& a.7,input2:& b.7}.get_result();
	EightBitMux{input1:& c,input2: & d,input3:& e,input4:& f,input5:& g,input6:& h,input7:& i,input8:& j}.get_result()
    }
}

///八位或
/// # Examples
/// ```
///    use algori::logicgate::EightBitOR;
///    let a = EightBitOR{input1: 80,input2:21}.get_result();
///    assert_eq!(a,85);
/// ```
pub struct EightBitOR{
    pub input1: i8,
    pub input2: i8,
}

impl EightBitOR {
    pub fn get_result(&self) -> i8 {
	let a = EightBitSplitter{input: self.input1}.get_result();
	let b = EightBitSplitter{input: self.input2}.get_result();
	let c = OR{input1:& a.0,input2:& b.0}.get_result();
	let d = OR{input1:& a.1,input2:& b.1}.get_result();
	let e = OR{input1:& a.2,input2:& b.2}.get_result();
	let f = OR{input1:& a.3,input2:& b.3}.get_result();
	let g = OR{input1:& a.4,input2:& b.4}.get_result();
	let h = OR{input1:& a.5,input2:& b.5}.get_result();
	let i = OR{input1:& a.6,input2:& b.6}.get_result();
	let j = OR{input1:& a.7,input2:& b.7}.get_result();
	EightBitMux{input1:& c,input2: & d,input3:& e,input4:& f,input5:& g,input6:& h,input7:& i,input8:& j}.get_result()
    }
}


///八位数据选择器
/// # 当input1为false时输出input2, input1为true输出input3
/// # Examples
/// ```
/// use algori::logicgate::DataSelector;
/// let a = DataSelector{input1:&Some(false),input2:20,input3:10}.get_result();
/// assert_eq!(a,20);
/// let a = DataSelector{input1:&Some(true),input2:20,input3:10}.get_result();
/// assert_eq!(a,10);
///```
pub struct DataSelector<'a>{
    pub input1: &'a Option<bool>,
    pub input2: i8,
    pub input3: i8,
}

impl<'a> DataSelector<'a>{
    pub fn get_result(&self) -> i8 {
	if *self.input1 ==Some(true) {
	    return self.input3;
	}
	self.input2
    }
}

///三位解码器
/// # 当switch为true时,
/// # 真值表
/// input1| input2| input3| output
/// ---|---|---|----
/// false|false|false|1
/// true|false|false|2
/// false|true|false|3
/// true|true|false|4
/// false|false|true|5
/// true|false|true|6
/// false|true|true|7
/// true|true|true|8
/// # Examples
/// ```
/// use algori::logicgate::ThreeDecoder;
/// let a = ThreeDecoder{input1: &Some(false),input2:&Some(false),input3:&Some(false),switch:&Some(false)}.get_result();
/// assert_eq!(a,(Some(true),Some(false),Some(false),Some(false),Some(false),Some(false),Some(false),Some(false)));
/// let a = ThreeDecoder{input1: &Some(false),input2:&Some(false),input3:&Some(true),switch:&Some(false)}.get_result();
/// assert_eq!(a,(Some(false),Some(false),Some(false),Some(false),Some(true),Some(false),Some(false),Some(false)));
/// let a = ThreeDecoder{input1: &Some(false),input2:&Some(true),input3:&Some(true),switch:&Some(false)}.get_result();
/// assert_eq!(a,(Some(false),Some(false),Some(false),Some(false),Some(false),Some(false),Some(true),Some(false)));
/// ```
pub struct ThreeDecoder<'a> {
    pub input1: &'a Option<bool>,
    pub input2: &'a Option<bool>,
    pub input3: &'a Option<bool>,
    pub switch: &'a Option<bool>,
}

impl<'a> ThreeDecoder<'a> {
    pub fn get_result(&self) -> (Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>,Option<bool>) {
	match self.switch {
	    Some(true) => return (None,None,None,None,None,None,None,None),
	    _ => {
		let a = NOR{input1: self.input1,input2: self.input2}.get_result(); //input1,input2全为false
		let b = NOR{input1: self.input2,input2: self.input3}.get_result(); //input2,input3全为false
		let f = AND{input1: &a, input2: &b}.get_result(); //1
		//input1,2,3->false时激活1
		let c = AND{input1: self.input1, input2: &b}.get_result(); //2
		//input2,3->false激活2
		let d = NOR{input1: self.input1,input2:self.input3}.get_result(); //input1,input3全为false
		let e = AND{input1: &d,input2: self.input2}.get_result(); //3
		//input2->true,input1,3->false激活3
		let g = NAND{input1: self.input1,input2: self.input2}.get_result();
		let h = NOR{input1: &g,input2: self.input3}.get_result(); //4
		//input1,2->false激活5
		let i = AND{input1: &a,input2: self.input3}.get_result(); //5
		//input1,3->true激活6
		let j = NOT{input: self.input2 }.get_result();
		let k = ThreeAND{input1: self.input1, input2: &j, input3: self.input3}.get_result(); //6
		//input2,3->true激活7
		let l = NOT{input:self.input1}.get_result();
		let m = ThreeAND{input1: &l,input2: self.input2, input3: self.input3}.get_result();
		//input1,2,3->true激活8
		let n = ThreeAND{input1: self.input1,input2: self.input2,input3: self.input3}.get_result();
		(f,c,e,h,i,k,m,n)

	    }
	}
    }
}

///八位算术引擎
/// # Operation
/// 1. operation = 0 -> OR
/// 2. operation = 1 -> NAND
/// 3. operation = 2 -> NOR
/// 4. operation = 3 -> AND
/// 5. operation = 4 -> ADD
/// 6. operation = 5 -> SUB
///
/// # Examples
/// ```
/// use algori::logicgate::EightBitALU; 
/// let a = EightBitALU {operation: 0 , input1: -76, input2: 92}.get_result();
/// assert_eq!(a,-4);
/// ```
pub struct EightBitALU {
    ///操作码
    pub operation: i8,
    pub input1: i8,
    pub input2: i8,
}

 impl EightBitALU {
     pub fn get_result(&self) -> i8 {
 	let splitter = EightBitSplitter{input: self.operation}.get_result();
	let decoder = ThreeDecoder{switch:&None,input1: &splitter.0,input2:&splitter.1,input3:&splitter.2}.get_result();
	match decoder {
	    (Some(true),Some(false),Some(false), ..) => return EightBitOR{input1: self.input1,input2:self.input2}.get_result(),
	    (Some(false),Some(true),Some(false),..) => return EightBitNAND{input1: self.input1,input2:self.input2}.get_result(),
	    (Some(false),Some(false),Some(true),..) => return EightBitNOR{input1: self.input1,input2: self.input2}.get_result(),
	    (Some(true),Some(true),Some(false),..) => return EightBitAND{input1: self.input1,input2: self.input2}.get_result(),
	    (Some(false),Some(false),Some(true),..) => return EightBitAdder{input1: &None,input2 :self.input1,input3:self.input2}.get_result().0,
	    _ => return EightBitSubber{input1 :self.input1,input2:self.input2}.get_result(),
	}
    }
}
