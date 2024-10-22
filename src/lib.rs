//! # current_previous
//!
//! `current_previous` contains the `CurrentPrevious` struct, which tracks the
//! current and previous values that it has held.

#[derive(Clone, Copy, Debug)]
pub struct CurrentPrevious<T> {
	current: T,
	previous: Option<T>
}

impl <T> CurrentPrevious<T> {
	/// Creates a new `CurrentPrevious` holding the `initial` value as its
	/// `current` value. The `previous` value is initially `None`.
	///
	/// # Examples
	///
	/// ```
	/// # use current_previous::CurrentPrevious;
	/// let current_previous = CurrentPrevious::new(0);
	///
	/// assert_eq!(current_previous.current(), &0);
	/// assert_eq!(current_previous.previous(), None);
	/// ```
	pub fn new(initial: T) -> Self {
		return Self {
			current: initial,
			previous: None
		};
	}

	/// Gets a reference to the `current` value.
	pub fn current(&self) -> &T {
		return &self.current;
	}

	/// Gets an optional reference to the `previous` value.
	pub fn previous(&self) -> Option<&T> {
		return self.previous.as_ref();
	}

	/// Sets a new `current` value, replacing the `previous` value with the old
	/// `current` value.
	///
	/// # Examples
	///
	/// ```
	/// # use current_previous::CurrentPrevious;
	/// let mut current_previous = CurrentPrevious::new(0);
	///
	/// assert_eq!(current_previous.current(), &0);
	/// assert_eq!(current_previous.previous(), None);
	///
	/// current_previous.update(1);
	///
	/// assert_eq!(current_previous.current(), &1);
	/// assert_eq!(current_previous.previous(), Some(&0));
	/// ```
	pub fn update(&mut self, new: T) {
		self.previous = Some(std::mem::replace(&mut self.current, new));
	}

	/// Replaces `self` with a new `CurrentPrevious` constructed from the given
	/// `new` value.
	///
	/// # Examples
	///
	/// ```
	/// # use current_previous::CurrentPrevious;
	/// let mut current_previous = CurrentPrevious::new(0);
	///
	/// assert_eq!(current_previous.current(), &0);
	/// assert_eq!(current_previous.previous(), None);
	///
	/// current_previous.reset(1);
	///
	/// assert_eq!(current_previous.current(), &1);
	/// assert_eq!(current_previous.previous(), None);
	/// ```
	pub fn reset(&mut self, new: T) {
		*self = Self::new(new);
	}

	/// Sets the `previous` value to `None`.
	///
	/// # Examples
	///
	/// ```
	/// # use current_previous::CurrentPrevious;
	/// let mut current_previous = CurrentPrevious::new(0);
	///
	/// assert_eq!(current_previous.current(), &0);
	/// assert_eq!(current_previous.previous(), None);
	///
	/// current_previous.update(1);
	///
	/// assert_eq!(current_previous.current(), &1);
	/// assert_eq!(current_previous.previous(), Some(&0));
	///
	/// current_previous.clear_previous();
	///
	/// assert_eq!(current_previous.current(), &1);
	/// assert_eq!(current_previous.previous(), None);
	/// ```
	pub fn clear_previous(&mut self) {
		self.previous = None;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_current() {
		let current_previous = CurrentPrevious::new(0);

		assert_eq!(current_previous.current(), &0);
		assert_eq!(current_previous.previous(), None);
	}

	#[test]
	fn set_current_twice() {
		let mut current_previous = CurrentPrevious::new(0);

		current_previous.update(1);

		assert_eq!(current_previous.current(), &1);
		assert_eq!(current_previous.previous(), Some(&0));
	}

	#[test]
	fn set_current_thrice() {
		let mut current_previous = CurrentPrevious::new(0);

		current_previous.update(1);

		current_previous.update(2);

		assert_eq!(current_previous.current(), &2);
		assert_eq!(current_previous.previous(), Some(&1));
	}

	#[test]
	fn clone() {
		let current_previous = CurrentPrevious::new(0);

		let mut cloned_current_previous = current_previous.clone();

		assert_eq!(current_previous.current(), &0);
		assert_eq!(current_previous.previous(), None);

		assert_eq!(cloned_current_previous.current(), &0);
		assert_eq!(cloned_current_previous.previous(), None);

		cloned_current_previous.update(1);

		assert_eq!(current_previous.current(), &0);
		assert_eq!(current_previous.previous(), None);

		assert_eq!(cloned_current_previous.current(), &1);
		assert_eq!(cloned_current_previous.previous(), Some(&0));
	}

	#[test]
	fn debug_print() {
		let mut current_previous = CurrentPrevious::new(0);

		assert_eq!(format!("{current_previous:?}"), "CurrentPrevious { current: 0, previous: None }");

		current_previous.update(1);

		assert_eq!(format!("{current_previous:?}"), "CurrentPrevious { current: 1, previous: Some(0) }");
	}

	#[test]
	fn reset() {
		let mut current_previous = CurrentPrevious::new(0);

		current_previous.reset(1);

		assert_eq!(current_previous.current(), &1);
		assert_eq!(current_previous.previous(), None);
	}
}
