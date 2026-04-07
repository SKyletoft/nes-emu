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
