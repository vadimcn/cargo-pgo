#![cfg(not(feature = "disable"))]
#![allow(improper_ctypes)]

#[macro_use]
extern crate cpp;

// Adapted from llvm-profdata
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

raw {
  using namespace llvm;

  static void exitWithError(const Twine &Message, StringRef Whence = "",
                            StringRef Hint = "") {
    errs() << "error: ";
    if (!Whence.empty())
      errs() << Whence << ": ";
    errs() << Message << "\n";
    if (!Hint.empty())
      errs() << Hint << "\n";
  }

  static void exitWithError(Error E, StringRef Whence = "") {
    if (E.isA<InstrProfError>()) {
      handleAllErrors(std::move(E), [&](const InstrProfError &IPE) {
        instrprof_error instrError = IPE.get();
        StringRef Hint = "";
        if (instrError == instrprof_error::unrecognized_format) {
          // Hint for common error of forgetting -sample for sample profiles.
          Hint = "Perhaps you forgot to use the -sample option?";
        }
        exitWithError(IPE.message(), Whence, Hint);
      });
    }

    exitWithError(toString(std::move(E)), Whence);
  }

  static void exitWithErrorCode(std::error_code EC, StringRef Whence = "") {
    exitWithError(EC.message(), Whence);
  }

  static void handleMergeWriterError(Error E, StringRef WhenceFile = "",
                                    StringRef WhenceFunction = "",
                                    bool ShowHint = true) {
    if (!WhenceFile.empty())
      errs() << WhenceFile << ": ";
    if (!WhenceFunction.empty())
      errs() << WhenceFunction << ": ";

    auto IPE = instrprof_error::success;
    E = handleErrors(std::move(E),
                    [&IPE](std::unique_ptr<InstrProfError> E) -> Error {
                      IPE = E->get();
                      return Error(std::move(E));
                    });
    errs() << toString(std::move(E)) << "\n";

    if (ShowHint) {
      StringRef Hint = "";
      if (IPE != instrprof_error::success) {
        switch (IPE) {
        case instrprof_error::hash_mismatch:
        case instrprof_error::count_mismatch:
        case instrprof_error::value_site_count_mismatch:
          Hint = "Make sure that all profile data to be merged is generated "
                "from the same binary.";
          break;
        default:
          break;
        }
      }

      if (!Hint.empty())
        errs() << Hint << "\n";
    }
  }

  template<typename T>
  struct Slice {
    T* data;
    uintptr_t len;
  };
}

  fn merge_instr_profiles_impl(inputs: &[&str] as "Slice<Slice<char>>",
                              output: &str as "Slice<char>") -> bool as "bool" {
    llvm_shutdown_obj shutdown;

    StringRef OutputFileName(output.data, output.len);
    std::error_code EC;
    raw_fd_ostream Output(OutputFileName, EC, sys::fs::F_None);
    if (EC) {
      exitWithErrorCode(EC, OutputFileName);
      return false;
    }

    InstrProfWriter Writer(false);
    SmallSet<instrprof_error, 4> WriterErrorCodes;

    for (int i = 0; i < inputs.len; ++i) {
      StringRef InputFileName(inputs.data[i].data, inputs.data[i].len);
      auto ReaderOrErr = InstrProfReader::create(InputFileName);
      if (Error E = ReaderOrErr.takeError()) {
        exitWithError(std::move(E), InputFileName);
        return false;
      }

      auto Reader = std::move(ReaderOrErr.get());
      bool IsIRProfile = Reader->isIRLevelProfile();
      if (Writer.setIsIRLevelProfile(IsIRProfile)) {
        exitWithError("Merge IR generated profile with Clang generated profile.");
        return false;
      }

      for (auto &I : *Reader) {
        if (Error E = Writer.addRecord(std::move(I), 1)) {
          // Only show hint the first time an error occurs.
          instrprof_error IPE = InstrProfError::take(std::move(E));
          bool firstTime = WriterErrorCodes.insert(IPE).second;
          handleMergeWriterError(make_error<InstrProfError>(IPE), InputFileName,
                                I.Name, firstTime);
        }
      }
      if (Reader->hasError()) {
        exitWithError(Reader->getError(), InputFileName);
        return false;
      }
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
