pub trait SaturateInto<T> {
	fn saturate_into(self) -> T;
}

pub trait SaturateEq<T> where Self: SaturateInto<T> + Sized, T: PartialEq {
	fn saturate_eq(self, other: T) -> bool {
		return SaturateInto::saturate_into(self) == other;
	}
}

impl<T, T2> SaturateEq<T> for T2 where T2: SaturateInto<T> + Sized, T: PartialEq {
}

impl<T> SaturateInto<T> for T {
	fn saturate_into(self) -> T { self }
}
 
impl SaturateInto<u32> for usize {
	fn saturate_into(self) -> u32 { 
		if self > u32::MAX as usize {
			return u32::MAX;
		} else {
			return self as u32;
		}
	}
}

impl SaturateInto<u64> for usize {
	fn saturate_into(self) -> u64 { self as u64 }
}

impl SaturateInto<isize> for usize {
    fn saturate_into(self) -> isize {
		if self > isize::MAX as usize {
			return isize::MAX;
		} else {
			return self as isize;
		}
	}
}

impl SaturateInto<i32> for usize {
    fn saturate_into(self) -> i32 { 
		if self > i32::MAX as usize {
			return i32::MAX;
		} else {
			return self as i32;
		}
	}
}

impl SaturateInto<i64> for usize {
    fn saturate_into(self) -> i64 { 
		if self > i64::MAX as usize {
			return i64::MAX;
		} else {
			return self as i64;
		}
	}
}

impl SaturateInto<usize> for u32 { fn saturate_into(self) -> usize { self as usize } }

impl SaturateInto<u64> for u32 { fn saturate_into(self) -> u64 { self as u64 } }

impl SaturateInto<isize> for u32 {
    fn saturate_into(self) -> isize { 
		if self > isize::MAX as u32 {
			return isize::MAX;
		} else {
			return self as isize;
		}
	}
}

impl SaturateInto<i32> for u32 {
    fn saturate_into(self) -> i32 { 
		if self > i32::MAX as u32 {
			return i32::MAX;
		} else {
			return self as i32;
		}
	}
}

impl SaturateInto<i64> for u32 {
    fn saturate_into(self) -> i64 { 
		if self > i64::MAX as u32 {
			return i64::MAX;
		} else {
			return self as i64;
		}
	}
}

impl SaturateInto<usize> for u64 {
    fn saturate_into(self) -> usize { 
		if self > usize::MAX as u64 {
			return usize::MAX;
		} else {
			return self as usize;
		}
	}
}

impl SaturateInto<u32> for u64 {
    fn saturate_into(self) -> u32 { 
		if self > u32::MAX as u64 {
			return u32::MAX;
		} else {
			return self as u32;
		}
	}
}

impl SaturateInto<isize> for u64 {
    fn saturate_into(self) -> isize { 
		if self > isize::MAX as u64 {
			return isize::MAX;
		} else {
			return self as isize;
		}
	}
}

impl SaturateInto<i32> for u64 {
    fn saturate_into(self) -> i32 { 
		if self > i32::MAX as u64 {
			return i32::MAX;
		} else {
			return self as i32;
		}
	}
}

impl SaturateInto<i64> for u64 {
    fn saturate_into(self) -> i64 { 
		if self > i64::MAX as u64 {
			return i64::MAX;
		} else {
			return self as i64;
		}
	}
}

impl SaturateInto<usize> for isize {
	fn saturate_into(self) -> usize { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as usize;
		}
	}
}

impl SaturateInto<u32> for isize {
	fn saturate_into(self) -> u32 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u32;
		}
	}
}

impl SaturateInto<u64> for isize {
	fn saturate_into(self) -> u64 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u64;
		}
	}
}

impl SaturateInto<i32> for isize {
	fn saturate_into(self) -> i32 { 
		if i32::MIN as isize > self { 
			return i32::MIN;
		} 
		else if self > i32::MAX as isize { 
			return i32::MAX;
		} 
		else { 
			return self as i32;
		}
	}
}

impl SaturateInto<i64> for isize {
	fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for i32 {
	fn saturate_into(self) -> usize {
		if self < 0 {
			return 0;
		} else {
			return self as usize;
		}
	}
}

impl SaturateInto<u32> for i32 {
	fn saturate_into(self) -> u32 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u32;
		}
	}
}

impl SaturateInto<u64> for i32 {
	fn saturate_into(self) -> u64 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u64;
		}
	}
}

impl SaturateInto<isize> for i32 {
	fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i64> for i32 {
	fn saturate_into(self) -> i64 { 
		self as i64
	}
}

impl SaturateInto<usize> for i64 {
	fn saturate_into(self) -> usize { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as usize;
		}
	}
}

impl SaturateInto<u32> for i64 {
	fn saturate_into(self) -> u32 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u32;
		}
	}
}

impl SaturateInto<u64> for i64 {
	fn saturate_into(self) -> u64 { 
		if self < 0 { 
			return 0;
		} 
		else { 
			return self as u64;
		}
	}
}

impl SaturateInto<isize> for i64 {
	fn saturate_into(self) -> isize { 
		if isize::MIN as i64 > self { 
			return isize::MIN;
		} 
		else if self > isize::MAX as i64 { 
			return isize::MAX;
		} 
		else { 
			return self as isize;
		}
	}
}

impl SaturateInto<i32> for i64 {
	fn saturate_into(self) -> i32 { 
		if i32::MIN as i64 > self { 
			return i32::MIN;
		} 
		else if self > i32::MAX as i64 { 
			return i32::MAX;
		} 
		else { 
			return self as i32;
		}
	}
}