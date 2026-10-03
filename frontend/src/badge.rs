use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
	loop {}
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
	println!("{info}");
	loop {}
}
