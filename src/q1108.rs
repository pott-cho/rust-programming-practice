pub fn defang_i_paddr(address: &str) -> String {
    address.replace('.', "[.]")
}
