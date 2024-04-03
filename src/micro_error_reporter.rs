//! Micro Error Reporter
//!
//! MicroErrorReporter extends the ErrorReporter base class, with a single
//! virtual function `Report` (runtime polymorphism). This function
//! actually just does some snprintf, and then calls back to the static
//! function `DebugLog` which we implement in interop.rs

use crate::bindings::tflite;

cpp! {{
    #include "tensorflow/lite/micro/tflite_bridge/micro_error_reporter.h"
}}

#[repr(transparent)]
pub struct MicroErrorReporter(tflite::MicroErrorReporter);

impl MicroErrorReporter {
    /// Create a new instance of a MicroErrorReporter
    pub fn new() -> Self {
        // All that happens here is that the C++ compiler fills in the
        // const function pointer so runtime polymorphism can happen.
        //
        // See further unsafe usage in micro_interpreter.rs
        let micro_error_reporter = unsafe {
            cpp!([] -> tflite::MicroErrorReporter as "tflite::MicroErrorReporter" {
                tflite::MicroErrorReporter micro_error_reporter;

                return micro_error_reporter;
            })
        };

        Self(micro_error_reporter)
    }
}

impl Default for MicroErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}
