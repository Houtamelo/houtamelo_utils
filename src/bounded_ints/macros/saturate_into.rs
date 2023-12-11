pub(crate) trait SaturateInto<T> {
	fn saturate_into(self) -> T;
}

impl SaturateInto<usize> for usize {
	fn saturate_into(self) -> usize { self }
}

impl SaturateInto<u32> for usize {
	fn saturate_into(self) -> u32 { self as u32 }
}

impl SaturateInto<u64> for usize {
	fn saturate_into(self) -> u64 { self as u64 }
}

impl SaturateInto<isize> for usize {
    fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i32> for usize {
    fn saturate_into(self) -> i32 { self as i32 }
}

impl SaturateInto<i64> for usize {
    fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for u32 {
    fn saturate_into(self) -> usize { self as usize }
}

impl SaturateInto<u32> for u32 {
    fn saturate_into(self) -> u32 { self }
}

impl SaturateInto<u64> for u32 {
    fn saturate_into(self) -> u64 { self as u64 }
}

impl SaturateInto<isize> for u32 {
    fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i32> for u32 {
    fn saturate_into(self) -> i32 { self as i32 }
}

impl SaturateInto<i64> for u32 {
    fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for u64 {
    fn saturate_into(self) -> usize { self as usize }
}

impl SaturateInto<u32> for u64 {
    fn saturate_into(self) -> u32 { self as u32 }
}

impl SaturateInto<u64> for u64 {
    fn saturate_into(self) -> u64 { self }
}

impl SaturateInto<isize> for u64 {
    fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i32> for u64 {
    fn saturate_into(self) -> i32 { self as i32 }
}

impl SaturateInto<i64> for u64 {
    fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for isize {
	fn saturate_into(self) -> usize { if self < 0 { 0 } else { self as usize } }
}

impl SaturateInto<u32> for isize {
	fn saturate_into(self) -> u32 { if self < 0 { 0 } else { self as u32 } }
}

impl SaturateInto<u64> for isize {
	fn saturate_into(self) -> u64 { if self < 0 { 0 } else { self as u64 } }
}

impl SaturateInto<isize> for isize {
	fn saturate_into(self) -> isize { self }
}

impl SaturateInto<i32> for isize {
	fn saturate_into(self) -> i32 { self as i32 }
}

impl SaturateInto<i64> for isize {
	fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for i32 {
	fn saturate_into(self) -> usize { if self < 0 { 0 } else { self as usize } }
}

impl SaturateInto<u32> for i32 {
	fn saturate_into(self) -> u32 { if self < 0 { 0 } else { self as u32 } }
}

impl SaturateInto<u64> for i32 {
	fn saturate_into(self) -> u64 { if self < 0 { 0 } else { self as u64 } }
}

impl SaturateInto<isize> for i32 {
	fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i32> for i32 {
	fn saturate_into(self) -> i32 { self }
}

impl SaturateInto<i64> for i32 {
	fn saturate_into(self) -> i64 { self as i64 }
}

impl SaturateInto<usize> for i64 {
	fn saturate_into(self) -> usize { if self < 0 { 0 } else { self as usize } }
}

impl SaturateInto<u32> for i64 {
	fn saturate_into(self) -> u32 { if self < 0 { 0 } else { self as u32 } }
}

impl SaturateInto<u64> for i64 {
	fn saturate_into(self) -> u64 { if self < 0 { 0 } else { self as u64 } }
}

impl SaturateInto<isize> for i64 {
	fn saturate_into(self) -> isize { self as isize }
}

impl SaturateInto<i32> for i64 {
	fn saturate_into(self) -> i32 { self as i32 }
}

impl SaturateInto<i64> for i64 {
	fn saturate_into(self) -> i64 { self }
}
