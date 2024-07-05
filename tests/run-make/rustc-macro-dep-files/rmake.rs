// --emit dep-info used to print all macro-generated code it could
// find as if it was part of a nonexistent file named "proc-macro source",
// which is not a valid path. After this was fixed in #36776, this test checks
// that macro code is not falsely seen as coming from a different file in dep-info.
// See https://github.com/rust-lang/rust/issues/36625

use run_make_support::{fs_wrapper, rustc, target};

fn main() {
    rustc().input("foo.rs").run();
    rustc().input("bar.rs").target(target()).emit("dep-info").run();
    assert!(!fs_wrapper::read_to_string("bar.d").contains("proc-macro source"));
}
