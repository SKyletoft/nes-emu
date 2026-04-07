#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests;

use core::mem::MaybeUninit;

pub struct Lru<K: PartialEq, V, const L: usize = 64> {
	cache: MaybeUninit<[(K, V); L]>,
	size: usize,
}

impl<K, V, const L: usize> Lru<K, V, L>
where
	K: PartialEq,
{
	pub fn new() -> Self {
		assert_ne!(
			L, 0,
			"LRU cannot have size 0. This should be promoted to a type error when const generics are more stable"
		);
		Self {
			cache: MaybeUninit::uninit(),
			size: 0,
		}
	}

	fn active_cache(&self) -> &[(K, V)] {
		unsafe { &self.cache.assume_init_ref()[..self.size] }
	}

	fn active_cache_mut(&mut self) -> &mut [(K, V)] {
		unsafe { &mut self.cache.assume_init_mut()[..self.size] }
	}

	fn is_full(&self) -> bool {
		self.size == L
	}

	fn find(&self, key: &K) -> Option<usize> {
		self.active_cache().iter().position(|(k, _)| k == key)
	}

	pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)> {
		if let Some(idx) = self.find(&key) {
			let active = self.active_cache_mut();
			active[..=idx].rotate_right(1);
			debug_assert!(active[0].0 == key);
			let ret = core::mem::replace(&mut active[0], (key, value));
			Some(ret)
		} else if self.is_full() {
			let active = self.active_cache_mut();
			let ret = core::mem::replace(&mut active[L - 1], (key, value));
			active.rotate_right(1);
			Some(ret)
		} else {
			let last_idx = self.size;
			self.size += 1;
			let active = self.active_cache_mut();
			unsafe { (&raw mut active[last_idx]).write((key, value)) };
			active.rotate_right(1);
			None
		}
	}

	pub fn get(&mut self, key: &K) -> Option<&V> {
		Some(&*self.get_mut(key)?)
	}

	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		let idx = self.find(key)?;
		let active = self.active_cache_mut();
		active[..=idx].rotate_right(1);
		Some(&mut active[0].1)
	}
}

impl<K, V, const L: usize> Default for Lru<K, V, L>
where
	K: PartialEq,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<K: PartialEq, V, const L: usize> Drop for Lru<K, V, L> {
	fn drop(&mut self) {
		for kv in self
			.active_cache_mut()
			.iter_mut()
			.map(|kv| kv as *mut (K, V))
		{
			unsafe { core::ptr::drop_in_place(kv) };
		}
	}
}
