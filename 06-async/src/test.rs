use crate::spawn_blocking;
use std::sync::atomic::{AtomicU8, AtomicBool, Ordering};

#[test]
fn test() {
	static FLAG: AtomicBool = AtomicBool::new(false);

	spawn_blocking(async { 
		println!("Hello, world!");
		FLAG.store(true, Ordering::Relaxed);
	});

	assert_eq!(FLAG.load(Ordering::Relaxed), true);
}

#[test]
fn test2() {
	static FLAG: AtomicBool = AtomicBool::new(false);

	async fn foo() {
		println!("ur async smells");
		std::thread::sleep(std::time::Duration::from_secs(1));
		FLAG.store(true, Ordering::Relaxed);
	}

	spawn_blocking(foo());

	assert_eq!(FLAG.load(Ordering::Relaxed), true);
}

#[test]
fn test3() {
	static COUNT: AtomicU8 = AtomicU8::new(0);

	async fn foo() {
		COUNT.fetch_add(1, Ordering::Relaxed);
		println!("ur async smells");
	}

	spawn_blocking(async {
		println!("Hello, world!");
		COUNT.fetch_add(1, Ordering::Relaxed);
		foo().await;
	});

	assert_eq!(COUNT.load(Ordering::Relaxed), 2);
}
