#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    unsafe extern "C++" {
        include!("rsmlui-sys/include/Core.h");

        fn get_version() -> String;
    }
}

#[cfg(test)]
#[test]
fn test() {
    dbg!(ffi::get_version());
}
