#![cfg_attr(not(test), no_std)]

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

#[cfg(test)]
mod tests {
	use quickcheck::{Arbitrary, Gen};
	use quickcheck_macros::quickcheck;
	use uluru::LRUCache;

	use crate::Lru;

	#[derive(Clone, Debug)]
	enum LruOp<K, V> {
		Get(K),
		GetMut(K),
		Insert(K, V),
	}

	impl<K, V> Arbitrary for LruOp<K, V>
	where
		K: Arbitrary,
		V: Arbitrary,
	{
		fn arbitrary(g: &mut Gen) -> Self {
			let variant = u8::arbitrary(g) % 3;
			match variant {
				0 => LruOp::Get(K::arbitrary(g)),
				1 => LruOp::GetMut(K::arbitrary(g)),
				_ => LruOp::Insert(K::arbitrary(g), V::arbitrary(g)),
			}
		}
	}

	fn run_ops<K, V, const L: usize>(ops: Vec<LruOp<K, V>>, lru: &mut Lru<K, V, L>)
	where
		K: PartialEq,
	{
		for op in ops.into_iter() {
			match op {
				LruOp::Get(k) => {
					let _ = lru.get(&k);
				}
				LruOp::GetMut(k) => {
					let _ = lru.get_mut(&k);
				}
				LruOp::Insert(k, v) => {
					lru.insert(k, v);
				}
			}
		}
	}

	#[quickcheck]
	fn test_drop_double_drop(prefix: Vec<LruOp<u8, Box<i32>>>) {
		let mut lru: Lru<u8, Box<i32>, 3> = Lru::default();

		run_ops(prefix, &mut lru);

		lru.insert(1, Box::new(1));
		lru.insert(2, Box::new(2));
		lru.insert(3, Box::new(3));
	}

	#[quickcheck]
	fn lru_ops_dont_crash(ops: Vec<LruOp<u8, u8>>) {
		let mut lru: Lru<u8, u8, 4> = Lru::default();
		run_ops(ops, &mut lru);
	}

	#[quickcheck]
	fn lru_preserves_value_after_l_minus_1_inserts(key: u8, value: u8) -> bool {
		let mut lru: Lru<u8, u8, 4> = Lru::default();
		lru.insert(key, value);
		for i in 0..3 {
			let other_key = (key + i + 1) % 16;
			lru.insert(other_key, i);
		}
		lru.get(&key).is_some()
	}

	#[quickcheck]
	fn lru_evicts_after_l_inserts(key: u8, value: u8) -> bool {
		let mut lru: Lru<u8, u8, 4> = Lru::default();
		lru.insert(key, value);
		for i in 0..4 {
			let other_key = (key + i + 1) % 16;
			lru.insert(other_key, i);
		}
		lru.get(&key).is_none()
	}

	#[quickcheck]
	fn get_what_was_inserted(prefix: Vec<LruOp<u8, u8>>, k: u8, v: u8) {
		let mut lru: Lru<u8, u8, 4> = Lru::new();
		run_ops(prefix, &mut lru);

		lru.insert(k, v);
		let returned = lru.get(&k);

		assert_eq!(Some(&v), returned);
	}

	#[test]
	fn evict_correct_val() {
		let mut lru: Lru<u8, u8, 2> = Lru::new();
		lru.insert(1, 1);
		lru.insert(2, 2);
		lru.insert(3, 3);

		assert!(lru.active_cache_mut().iter().any(|(_, v)| *v == 2));
		assert!(lru.active_cache_mut().iter().any(|(_, v)| *v == 3));
		assert!(!lru.active_cache_mut().iter().any(|(_, v)| *v == 1));

		lru.insert(3, 1);
		assert!(lru.active_cache_mut().iter().any(|(_, v)| *v == 1));
		assert!(lru.active_cache_mut().iter().any(|(_, v)| *v == 2));
		assert!(!lru.active_cache_mut().iter().any(|(_, v)| *v == 3));
	}

	#[quickcheck]
	fn lru_matches_uluru_behaviour(ops: Vec<LruOp<u8, u8>>) {
		let mut our_lru: Lru<u8, u8, 4> = Lru::default();
		let mut uluru_lru: LRUCache<(u8, u8), 4> = LRUCache::new();

		for op in ops.iter() {
			match op {
				LruOp::Get(k) => {
					let our_result = our_lru.get(k).copied();
					let uluru_result = uluru_lru.find(|pair| pair.0 == *k).map(|pair| pair.1);
					assert_eq!(our_result, uluru_result);
				}
				LruOp::GetMut(k) => {
					let our_result = our_lru.get_mut(k).map(|v| *v);
					let uluru_result = uluru_lru.find(|pair| pair.0 == *k).map(|pair| pair.1);
					assert_eq!(our_result, uluru_result);
				}
				LruOp::Insert(k, v) => {
					let our_result = our_lru.insert(*k, *v);
					let uluru_result = uluru_lru.insert((*k, *v));
					assert_eq!(our_result, uluru_result);
				}
			}
		}
	}
}
