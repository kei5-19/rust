fn main() {
    unsafe { *std::ptr::null_mut() = 0i32 }; //~ ERROR constant evaluation error: invalid use of NULL pointer
}
