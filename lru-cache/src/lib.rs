use std::mem::MaybeUninit;

pub struct Lru<K: PartialEq, V, const L: usize = 64> {
	cache: MaybeUninit<[(K, V); L]>,
	size: usize,
}

impl<K, V, const L: usize> Lru<K, V, L>
where
	K: PartialEq,
{
	fn new() -> Self {
		Self {
			cache: MaybeUninit::uninit(),
			size: 0,
		}
	}

	fn active_cache_mut(&mut self) -> &mut [(K, V)] {
		unsafe { &mut self.cache.assume_init_mut()[..self.size] }
	}

	pub fn insert(&mut self, key: K, value: V) {
		if self.size != L {
			self.size += 1;
		}
		let last = self.size - 1;
		let active = self.active_cache_mut();
		active[last] = (key, value);
		active.rotate_right(1);
	}

	pub fn get(&mut self, key: &K) -> Option<&V> {
		Some(&*self.get_mut(key)?)
	}

	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		let size = self.size;
		let active = self.active_cache_mut();
		let idx = active.iter().take(size).position(|(k, _)| k == key)?;
		active[..idx].rotate_right(1);
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
			drop(unsafe { std::ptr::read(kv) });
		}
	}
}

#[cfg(test)]
mod tests {
	use quickcheck::{Arbitrary, Gen, quickcheck};
	use uluru::LRUCache;

	#[derive(Clone, Debug)]
	enum LruOp<K, V> {
		Get(K),
		GetMut(K),
		Insert(K, V),
	}

	impl<K: Arbitrary, V: Arbitrary> Arbitrary for LruOp<K, V> {
		fn arbitrary(g: &mut Gen) -> Self {
			let variant = u8::arbitrary(g) % 3;
			match variant {
				0 => LruOp::Get(K::arbitrary(g)),
				1 => LruOp::GetMut(K::arbitrary(g)),
				_ => LruOp::Insert(K::arbitrary(g), V::arbitrary(g)),
			}
		}
	}

	fn run_ops<const L: usize>(ops: &[LruOp<u8, u8>], lru: &mut super::Lru<u8, u8, L>) {
		for op in ops {
			match op {
				LruOp::Get(k) => {
					let _ = lru.get(k);
				}
				LruOp::GetMut(k) => {
					let _ = lru.get_mut(k);
				}
				LruOp::Insert(k, v) => {
					lru.insert(*k, *v);
				}
			}
		}
	}

	quickcheck! {
		fn lru_ops_dont_crash(ops: Vec<LruOp<u8, u8>>) -> bool {
			dbg!(&ops);
			let mut lru: super::Lru<u8, u8, 4> = super::Lru::default();
			run_ops(&ops, &mut lru);
			true
		}

		fn lru_preserves_value_after_l_minus_1_inserts(key: u8, value: u8) -> bool {
			let mut lru: super::Lru<u8, u8, 4> = super::Lru::default();
			lru.insert(key, value);
			for i in 0..3 {
				let other_key = (key + i + 1) % 16;
				lru.insert(other_key, i);
			}
			lru.get(&key).is_some()
		}

		fn lru_evicts_after_l_inserts(key: u8, value: u8) -> bool {
			let mut lru: super::Lru<u8, u8, 4> = super::Lru::default();
			lru.insert(key, value);
			for i in 0..4 {
				let other_key = (key + i + 1) % 16;
				lru.insert(other_key, i);
			}
			lru.get(&key).is_none()
		}

		fn lru_matches_uluru_behavior(ops: Vec<LruOp<u8, u8>>) -> bool {
			let mut our_lru: super::Lru<u8, u8, 4> = super::Lru::default();
			let mut uluru_lru: LRUCache<(u8, u8), 4> = LRUCache::new();

			for op in ops.iter() {
				match op {
				LruOp::Get(k) => {
					let our_result = our_lru.get(k).copied();
					let uluru_result = uluru_lru
						.find(|pair| pair.0 == *k)
						.map(|pair| pair.1);
					if our_result != uluru_result {
						return false;
					}
				}
				LruOp::GetMut(k) => {
					let our_result = our_lru.get_mut(k).map(|v| *v);
					let uluru_result = uluru_lru
						.find(|pair| pair.0 == *k)
						.map(|pair| pair.1);
					if our_result != uluru_result {
						return false;
					}
				}
					LruOp::Insert(k, v) => {
						our_lru.insert(*k, *v);
						uluru_lru.insert((*k, *v));
					}
				}
			}
			true
		}
	}
}
