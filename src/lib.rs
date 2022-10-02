#[macro_export]
macro_rules! type_macro {
    ( $arg1: ty => $arg2:ident ) => {
        type $arg2 = $arg1;
    };
}
#[macro_export]
macro_rules! append_mac {
    ($element:expr) => {{
        //2d {} makes this block into expression (as macros expect to return expressions)
        // if we have valid single line expressions 2nd {} is not needed
        let mut col = Vec::new();
        col.push($element);
        col.push($element + 1)
    }};
}

#[macro_export]
///https://doc.rust-lang.org/std/macro.vec.html
///```
/// {
///     let mut col = Vec::new();
///     col.push(43);
///     col.push(44);
///     col.push(45);
/// }
///```
macro_rules! match_input {
    ($($x:expr),+) => {{// someting like regex exptession (match one or more param delimited by ',')
        let mut col = Vec::new();
        $(col.push($x);)+ // + marks ita as repetition (can also be * or , or ?)
        //when this is hit it looks up what pattern it's matching against and pulls out variables
        //above line can be repeated to do expansion multiple times
    }};
}
#[macro_export]
///if you use VARIABLES (line 43) for multiple different repeating paterns
/// they have to repeat the same number of times
macro_rules! multiple_dif_paterns {
    ($($element:expr),+; $($id:ident),+) => {{
        let mut col = Vec::new();
        $(let $id = col.push($element);)+
    }};
}
#[macro_export]
///``` bellow $(,)? means > following the initial patern we want to allow zero or 1 trailing comma
///($($x:expr),+ $(,)?) => { ... };
///
/// also bellow ',' inside $() means we require trailing comma
/// ($($x:exp,)+) => { ... };
///
/// also bellow is valid (" " is default whitespace separator)
/// $($x:expr)+ $(,)?) => { ... }
/// ```
macro_rules! optional_trailing_comma {
    ($($x:expr),+ $(,)?) => {{
        let mut col = Vec::new();
        $(col.push($x);)+
        col
    }};
}

//Usefull macro implementation

trait MaxValue {
    fn max_value() -> Self;
}

#[macro_export]
///Get max value of passed type
macro_rules! impl_max {
    ($t:ty) => {{
        impl $crate::MaxValue for $t {
            //ceate ALWAYS REFRES TO ONE WHERE MACRO WAS DEFINED
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    }};
}

#[macro_export]
macro_rules! with_count {
    ($x:expr; $count:expr) => {{
        let mut col = Vec::new();
        let element = $x; //eval expr only once
        for _ in 0..$count {
            col.push(element.clone());
        }
        col
    }};
}

#[macro_export]
macro_rules! efficient_with_count {
    ($x:expr; $count:expr) => {{
        let count = $count;
        let mut col = Vec::with_capacity(count);
        col.resize(count, $x); //no bound checkingm here
                               //or
                               //col.extend(std::iter::repeat($x).take(count));
                               //this is less efficient because of bounds checking
        col
    }};
}

//HACK
#[macro_export]
///see: https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length
macro_rules! count_without_param {
    ($($x:expr),+) => {{
        //check that count is const
        const _:usize = $crate::count![@COUNT; $($x),+];

        #[allow(unused_mut)]
        let mut col = Vec::with_capacity($crate::count![@COUNT; $($x),+]);
        $(col.push($x);)+
        col
    }};
}

//WE CAN ALSO HIDE THOSE NASTY PATTERNS LIKE
//cargo doc --open (opens html generated docs)
#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($x:expr),+) => {
        <[()]>::len(&[$($crate::count![@SUBST; $x]),+])
    };
    (@SUBST; $_element:expr) => {()};
}

// --------------------test functions------------------------
//cargo expand --lib --tests     (to expand all test's macros)
//cargo expand --lib --tests "push_num_to_macro"
//cargo test                     (run all tests)
//cargo test "push_num_to_macro" (run only specified test)
//cargo test --help              (more options)
//to see println "cargo test "type_max" -- --nocapture"
//https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests

//NOTE :ERROR Propagation
//if we pass param that doesnt satisfy some type traite error
//gets bubbled up to the top from generated expression code , and points to param it came from

//Usefull MACRO PATTERNS

#[test]
fn count_without_param() {
    let _: Vec<u32> = count_without_param! {1,2,3};
}

#[test]
fn efficient_eval_expr_once() {
    let mut y = Some(42);
    let x: Vec<u32> = efficient_with_count! {y.take().unwrap();2};
    assert_eq!(x.len(), 2);
}
//we can do even better if this implemented exact size iterator
//that way we could do all the bounds checking in advance

#[test]
fn eval_expr_once() {
    let mut y = Some(42);
    let x: Vec<u32> = with_count! {y.take().unwrap();2};
    //bellow macros will also be expanded
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn trailing_comma() {
    let _: Vec<&'static str> = optional_trailing_comma! {
        "asasjajsjaskakskaskakska",
        "asasjajsjaskakskaskakska",
        "asasjajsjaskakskaskakska",
        "asasjajsjaskakskaskakska"
    };
}

#[test]
fn multi_input_variable() {
    multiple_dif_paterns! {43;_ok}
    //or
    multiple_dif_paterns! {43,44;_ok,_ok2}
}

#[test]
fn input_syntax() {
    match_input! {43,44,45}
}
#[test]
fn push_num_to_macro() {
    append_mac! {42}
}

#[test]
fn type_macro() {
    type_macro! {u32 => Okayyy}
}

#[test]
fn type_max() {
    println!("{}", u32::MAX);
    impl_max!(u32)
}
