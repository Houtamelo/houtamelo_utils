macro_rules! bound_main {
	($type_name:ty, $inner:ty) => {
		impl<const MIN: $inner, const MAX: $inner>  $type_name {
			pub const fn new(inner_value: $inner) -> Self {
				if inner_value >= MAX {
					return Self {
						inner_value: MAX
					};
				} else if inner_value <= MIN {
					return Self {
						inner_value: MIN
					};
				} else {
					return Self {
						inner_value
					};
				}
			}

			pub const fn get(&self) -> $inner { return self.inner_value; }

			pub fn set(&mut self, value: $inner) { self.inner_value = value.clamp(MIN, MAX); }
		}

		impl<const MIN: $inner, const MAX: $inner> Default for $type_name {
			fn default() -> Self { return Self::new(MIN); }
		}

		impl<const MIN: $inner, const MAX: $inner> std::hash::Hash for $type_name {
			fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
		}
		
		impl<const MIN: $inner, const MAX: $inner> core::ops::Deref for $type_name { 
			type Target = $inner;
			
			fn deref(&self) -> &Self::Target { 
				return &self.inner_value; 
			} 
		}
	};
}

pub(crate) use bound_main;