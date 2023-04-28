pub fn invalid_address<T>(address_type: &str, address: u16) -> T {
    panic!("Invalid {} address 0x{:4X}", address_type, address)
}
