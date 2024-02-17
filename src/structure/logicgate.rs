///逻辑门
pub trait LogicGate {
    fn get_result(&self) -> bool;
}



///与非门 全为true则为false
/// # Examples
/// ```
/// use algori::structure::Nand;
/// use algori::structure::LogicGate;
/// let a = Nand{input1: true, input2: false};
/// assert_eq!(a.get_result(),true)
/// ```
#[derive(PartialEq)]
pub struct Nand {
    pub input1: bool,
    pub input2: bool,
}


impl LogicGate for Nand {

    fn get_result(&self) -> bool {
	if self.input1 == true{
	    if self.input2 == true {
		return false;
	    }
	}
	true
    }
}

///非门 反转真值表
///非门由一个输入一致的与非门组成
/// # Examples
/// ```
/// use algori::structure::Not;
/// use algori::structure::LogicGate;
/// let a = Not{input: true};
/// let b = Not{input: false};
/// assert_eq!(a.get_result(),false);
/// assert_eq!(b.get_result(),true);
/// ```
pub struct Not {
    pub input: bool,
}


impl LogicGate for Not {
    fn get_result(&self) -> bool {
	let a = Nand{input1: self.input, input2: self.input};
	a.get_result()
    }
}

