macro_rules! tests_base {
    ($f: ident, $b: expr, $n100: expr) => {
        mod $f {
            use std::fmt::Write;
            use ::*;

            #[test]
            fn zero() {
                let r = Radix::new(0, $b);
                let mut s = String::new();

                write!(&mut s, "{}", r).unwrap();
                assert_eq!(s, "0");
            }

            #[test]
            fn hundred() {
                let r = Radix::new(100, $b);
                let mut s = String::new();

                write!(&mut s, "{}", r).unwrap();
                assert_eq!(s, $n100);
            }
        }
    };
}

tests_base!(base_2, 2, "1100100");
tests_base!(base_3, 3, "10201");
tests_base!(base_4, 4, "1210");
tests_base!(base_5, 5, "400");
tests_base!(base_6, 6, "244");
tests_base!(base_7, 7, "202");
tests_base!(base_8, 8, "144");
tests_base!(base_9, 9, "121");
tests_base!(base_10, 10, "100");
tests_base!(base_11, 11, "91");
tests_base!(base_12, 12, "84");
tests_base!(base_13, 13, "79");
tests_base!(base_14, 14, "72");
tests_base!(base_15, 15, "6a");
tests_base!(base_16, 16, "64");
tests_base!(base_17, 17, "5f");
tests_base!(base_18, 18, "5a");
tests_base!(base_19, 19, "55");
tests_base!(base_20, 20, "50");
tests_base!(base_21, 21, "4g");
tests_base!(base_22, 22, "4c");
tests_base!(base_23, 23, "48");
tests_base!(base_24, 24, "44");
tests_base!(base_25, 25, "40");
tests_base!(base_26, 26, "3m");
tests_base!(base_27, 27, "3j");
tests_base!(base_28, 28, "3g");
tests_base!(base_29, 29, "3d");
tests_base!(base_30, 30, "3a");
tests_base!(base_31, 31, "37");
tests_base!(base_32, 32, "34");
tests_base!(base_33, 33, "31");
tests_base!(base_34, 34, "2w");
tests_base!(base_35, 35, "2u");
tests_base!(base_36, 36, "2s");


macro_rules! test_shortcut {
    ($f: ident, $b: expr) => {
        mod $f {
            use std::fmt::Write;
            use ::*;

            #[test]
            fn shortcut() {
                let r = Radix::new(1234, $b);
                let mut s1 = String::new();
                let mut s2 = String::new();

                write!(&mut s1, "{}", r).unwrap();
                write!(&mut s2, "{}", $f(1234)).unwrap();
                assert_eq!(s1, s2);
            }
        }
    }
}

test_shortcut!(radix_3, 3);
test_shortcut!(radix_4, 4);
test_shortcut!(radix_5, 5);
test_shortcut!(radix_6, 6);
test_shortcut!(radix_7, 7);
test_shortcut!(radix_9, 9);
test_shortcut!(radix_11, 11);
test_shortcut!(radix_12, 12);
test_shortcut!(radix_13, 13);
test_shortcut!(radix_14, 14);
test_shortcut!(radix_15, 15);
test_shortcut!(radix_17, 17);
test_shortcut!(radix_18, 18);
test_shortcut!(radix_19, 19);
test_shortcut!(radix_20, 20);
test_shortcut!(radix_21, 21);
test_shortcut!(radix_22, 22);
test_shortcut!(radix_23, 23);
test_shortcut!(radix_24, 24);
test_shortcut!(radix_25, 25);
test_shortcut!(radix_26, 26);
test_shortcut!(radix_27, 27);
test_shortcut!(radix_28, 28);
test_shortcut!(radix_29, 29);
test_shortcut!(radix_30, 30);
test_shortcut!(radix_31, 31);
test_shortcut!(radix_32, 32);
test_shortcut!(radix_33, 33);
test_shortcut!(radix_34, 34);
test_shortcut!(radix_35, 35);
test_shortcut!(radix_36, 36);

use std::fmt::Write;
use ::*;

#[test]
fn alternate_capitalize() {
    let r = Radix::new(100, 28);
    let mut s = String::new();

    write!(&mut s, "{:#}", r).unwrap();
    assert_eq!(s, "3G");
}