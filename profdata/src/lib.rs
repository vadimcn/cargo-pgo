#![allow(improper_ctypes)]

#[macro_use]
extern crate cpp;

cpp! {

#include "llvm/ADT/SmallSet.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/ProfileData/InstrProfReader.h"
#include "llvm/ProfileData/InstrProfWriter.h"
#include "llvm/ProfileData/ProfileCommon.h"
#include "llvm/ProfileData/SampleProfReader.h"
#include "llvm/ProfileData/SampleProfWriter.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Format.h"
#include "llvm/Support/ManagedStatic.h"
#include "llvm/Support/MemoryBuffer.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/PrettyStackTrace.h"
#include "llvm/Support/Signals.h"
#include "llvm/Support/raw_ostream.h"
#include <algorithm>

#include "rust_types.h"


fn merge_instr_profiles_impl(inputs: &[&str] as "rs::Slice<rs::Slice<char>>", 
                             output: &str as "rs::Slice<char>") -> bool as "bool" {
  using namespace llvm;

  std::error_code EC;
  raw_fd_ostream Output(StringRef(output.data, output.len), EC, sys::fs::F_None);
  if (EC)
    return false;

  InstrProfWriter Writer(false);
  SmallSet<instrprof_error, 4> WriterErrorCodes;

  for (int i = 0; i < inputs.len; ++i) {
    StringRef InputFileName(inputs.data[i].data, inputs.data[i].len);
    int InputWeight = 1;
    auto ReaderOrErr = InstrProfReader::create(InputFileName);
    if (Error E = ReaderOrErr.takeError())
      return false;

    auto Reader = std::move(ReaderOrErr.get());
    bool IsIRProfile = Reader->isIRLevelProfile();
    if (Writer.setIsIRLevelProfile(IsIRProfile))
      return false;

    for (auto &I : *Reader) {
      if (Error E = Writer.addRecord(std::move(I), InputWeight)) {
          return false;
      }
    }
    if (Reader->hasError())
      return false;
  }
  Writer.write(Output);
  return true;
}

}

pub fn merge_instr_profiles(inputs: &[&str], output: &str) -> bool {
    unsafe {
        merge_instr_profiles_impl(inputs, output)
    }
} 

/*
#[repr(C)]
struct CStringRef {
  ptr: *const c_char,
  len: c_int,
}

extern "C" {
    fn  _merge_instr_profiles(
        num_inputs: c_int,
        inputs: *const CStringRef,
        output: CStringRef) -> c_bool;
}

pub fn merge_instr_profiles(inputs: &[&str], output: &str) -> bool {
    unsafe {
        let inputs: Vec<*const c_char> = inputs.collect(); 
        _merge_instr_profiles(
            inputs.len(),
            &inputs, 
            CStringRef { ptr: output.as_ptr(), len: output.len() })  
    }
}
*/