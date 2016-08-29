extern crate nested_test;

#[allow(non_camel_case_types)]
pub enum nsICSSPseudoElement {}
extern  {
    #[link_name = "?before@nsCSSPseudoElements@@2PAVnsICSSPseudoElement@@A"]
    pub static nsCSSPseudoElements_before: *mut nsICSSPseudoElement;
}

#[no_mangle]
pub extern "C" fn test() {
    println!("{}", nested_test::nsCSSPseudoElements_after as usize);
    println!("{}", nsCSSPseudoElements_before as usize);
}
