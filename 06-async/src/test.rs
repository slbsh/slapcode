use crate::Runtime;

#[test]
fn test() {
	Runtime::spawn_blocking(async { println!("Hello, world!"); });
}

#[test]
fn test2() {
	async fn foo() {
		println!("ur async smells");
	}

	Runtime::spawn_blocking(foo());
}

#[test]
fn test3() {
	async fn foo() {
		println!("ur async smells");
	}

	Runtime::spawn_blocking(async {
		println!("Hello, world!");
		foo().await;
	});
}
