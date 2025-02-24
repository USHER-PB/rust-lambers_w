#[cfg(test)]
mod tests {
   use lambers_w::input::take_value;

    #[test]

     fn test_invalid_inputs(){
      assert!(take_value(-0.3).is_ok());
      assert!(take_value(-0.6).is_err());
      assert!(take_value(-0.9).is_err());
     }

}
