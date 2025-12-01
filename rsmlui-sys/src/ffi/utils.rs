// use std::mem::MaybeUninit;
// #[cxx::bridge]
// mod ffi {
//     #[namespace = "rsmlui::utils::options"]
//     unsafe extern "C++" {
//         include!("rsmlui/Utils.h");

//         type RsmlOptionString;

//         unsafe fn make_string_option_none(out: *mut RsmlOptionString);
//         unsafe fn make_string_option_some(out: *mut RsmlOptionString, value: String);
//     }
// }

// pub(crate) use ffi::RsmlOptionString;

// use crate::utils::ffi::{make_string_option_none, make_string_option_some};

// impl From<Option<String>> for RsmlOptionString {
//     fn from(value: Option<String>) -> RsmlOptionString {
//         match value {
//             None => {
//                 let mut out = MaybeUninit::<RsmlOptionString>::uninit();
//                 unsafe {
//                     make_string_option_none(out.as_mut_ptr());
//                 }
//                 unsafe { out.assume_init() }
//             },
//             Some(value) => {
//                 let mut out = MaybeUninit::<RsmlOptionString>::uninit();
//                 unsafe {
//                     make_string_option_some(out.as_mut_ptr(), value);
//                 }
//                 unsafe { out.assume_init() }
//             },
//         }
//     }
// }
