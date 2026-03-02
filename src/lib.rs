#![feature(maybe_uninit_slice)]

pub mod app;
mod possible;
pub mod sheet;
pub mod tables;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

// macro_rules! basic {
//     (
//         types { $( $variant:ident : $real:ty ),* $(,)? }
//     ) => {
//         $(
//             println!("{}: {}", stringify!($variant), stringify!($real));
//         )*
//     };
// }

// basic! {
//     types {
//         U8: u8,
//         U16: u16,
//     }
// }

// macro_rules! typed {
//     (
//         types { $( $variant:ident : $real:ty ),* $(,)? }

//         $(
//             $(#[$meta:meta])*
//             $vis:vis struct $name:ident $( < $($generics:tt)* > )? $body:tt $( where $( $where:tt )* )? $(;)?
//         )*
//     ) => {
//         $(
//             $(#[$meta])*
//             $vis enum $name {
//                 $(
//                     $variant typed!(@body $real $( ( $($tuple)* ) )? $( { $($fields)* } )? )
//                 ),*
//             }
//         )*
//     };

//     // ---------- body forms ----------

//     (@body $real:ty) => {};

//     (@body $real:ty ( $($tuple:tt)* )) => {
//         ( typed!(@replace $real ; $($tuple)*) )
//     };

//     (@body $real:ty { $($fields:tt)* }) => {
//         { typed!(@replace $real ; $($fields)*) }
//     };

//     // ---------- token replacer ----------

//     (@replace $real:ty ; ) => {};

//     (@replace $real:ty ; T $($rest:tt)*) => {
//         $real typed!(@replace $real ; $($rest)*)
//     };

//     (@replace $real:ty ; $other:tt $($rest:tt)*) => {
//         $other typed!(@replace $real ; $($rest)*)
//     };
// }

// typed! {
//     types {
//         B: u8,
//         U16: u16,
//     }

//     #[derive(Clone, Debug, Serialize, Deserialize)]
//     pub struct Type;

//     pub struct Value(T);

//     #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
//     pub struct ValueVec(Vec<T>);

//     #[derive(Debug)]
//     pub struct StructWithValue {
//         value: T,
//         name: String,
//     }
// }

// should generate

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum Type {
//     U8,
//     U16,
// }

// pub enum Value {
//     U8(u8),
//     U16(u16),
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// pub enum ValueVec {
//     U8(Vec<u8>),
//     U16(Vec<u16>),
// }

// #[derive(Debug)]
// pub enum StructWithValue {
//     U8 { value: u8, name: String },
//     U16 { value: u16, name: String },
// }

// pub struct A<T, A>(T, A)
// where
//     T: Copy,
//     U: Clone;

// pub struct B<T, U>
// where
//     T: Copy,
//     U: Clone,
// {
//     t: T,
//     u: U,
// }

// pub struct C<T, U>
// where
//     T: Copy,
//     U: Clone,
// {
//     t: T,
//     u: U,
// }

macro_rules! typed {
    (
        types { $( $var:ident : $real:ty ),* $(,)? }

        $(
            $( #[$meta:meta] )*
            $vis:vis struct $name:ident $( < $($gen:tt)* > )? ( $($body:tt)* ) $( where $($where:tt)* )? ;
        )*
    ) => {};

    (
        types { $( $var:ident : $real:ty ),* $(,)? }

        $(
            $( #[$meta:meta] )*
            $vis:vis struct $name:ident $( < $($gen:tt)* > )? $( where $($where:tt)* )? $body:tt
        )*
    ) => {};
}

macro_rules! struct_patterns {
    // unit
    (
        $vis:vis struct $name:ident;
    ) => {
        $vis enum $name {

        }
    };

    // tuple
    (
        $vis:vis struct $name:ident $( < $($gen:tt)* > )? ( $($body:tt)* ) $( where $($where:tt)* )? ;
    ) => {};

    // named
    (
        $vis:vis struct $name:ident $( < $($gen:tt)* > )? $( where $($where:tt)* )? $body:tt
    ) => {};
}

struct F<T>
where
    T: Copy;
