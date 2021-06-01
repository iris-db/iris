use core::mem;
use std::sync::{Arc, Mutex, Once};

pub struct SingletonPointer {
	inner: Arc<Mutex<u8>>,
}

pub unsafe fn alloc_singleton<T>(s: T) -> *const SingletonPointer {
	let mut outer_singleton: *const SingletonPointer = 0 as *const SingletonPointer;
	let once: Once = Once::new();

	once.call_once(|| {
		let singleton = SingletonPointer {
			inner: Arc::new(Mutex::new(0)),
		};

		outer_singleton = mem::transmute(Box::new(singleton));
	});

	outer_singleton
}
