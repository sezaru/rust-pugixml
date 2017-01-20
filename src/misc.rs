pub fn version() -> i32 {
    use wrapper::pugi_version;
    unsafe {
        return pugi_version();
    }
}
