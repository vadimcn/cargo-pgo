#![allow(improper_ctypes)]

extern "C" {
  fn merge_instr_profiles_impl(inputs: &[&str], output: &str) -> bool;
}

pub fn merge_instr_profiles(inputs: &[&str], output: &str) -> bool {
  unsafe {
    merge_instr_profiles_impl(inputs, output)
  }
}
