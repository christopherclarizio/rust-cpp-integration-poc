mod hello_world {
  #[no_mangle]
  pub extern "C" fn print_hello_world() {
    println!("Hello, world from Rust library!");
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
