use crate::*;
use fluid::prelude::*;

#[theory]
#[case(2, "1100100")]
#[case(3, "10201")]
#[case(4, "1210")]
#[case(5, "400")]
#[case(6, "244")]
#[case(7, "202")]
#[case(8, "144")]
#[case(9, "121")]
#[case(10, "100")]
#[case(11, "91")]
#[case(12, "84")]
#[case(13, "79")]
#[case(14, "72")]
#[case(15, "6a")]
#[case(16, "64")]
#[case(17, "5f")]
#[case(18, "5a")]
#[case(19, "55")]
#[case(20, "50")]
#[case(21, "4g")]
#[case(22, "4c")]
#[case(23, "48")]
#[case(24, "44")]
#[case(25, "40")]
#[case(26, "3m")]
#[case(27, "3j")]
#[case(28, "3g")]
#[case(29, "3d")]
#[case(30, "3a")]
#[case(31, "37")]
#[case(32, "34")]
#[case(33, "31")]
#[case(34, "2w")]
#[case(35, "2u")]
#[case(36, "2s")]
fn base_formatting(base: u32, hundred_formatted: &str) {
    let zero = Radix::new(0, base);
    let hundred = Radix::new(100, base);

    format!("{}", zero).should().be_equal_to("0");
    format!("{}", hundred)
        .should()
        .be_equal_to(hundred_formatted);
}

#[theory]
#[case(radix_3, 3)]
#[case(radix_4, 4)]
#[case(radix_5, 5)]
#[case(radix_6, 6)]
#[case(radix_7, 7)]
#[case(radix_9, 9)]
#[case(radix_11, 11)]
#[case(radix_12, 12)]
#[case(radix_13, 13)]
#[case(radix_14, 14)]
#[case(radix_15, 15)]
#[case(radix_17, 17)]
#[case(radix_18, 18)]
#[case(radix_19, 19)]
#[case(radix_20, 20)]
#[case(radix_21, 21)]
#[case(radix_22, 22)]
#[case(radix_23, 23)]
#[case(radix_24, 24)]
#[case(radix_25, 25)]
#[case(radix_26, 26)]
#[case(radix_27, 27)]
#[case(radix_28, 28)]
#[case(radix_29, 29)]
#[case(radix_30, 30)]
#[case(radix_31, 31)]
#[case(radix_32, 32)]
#[case(radix_33, 33)]
#[case(radix_34, 34)]
#[case(radix_35, 35)]
#[case(radix_36, 36)]
fn verify_shortcut(f: fn(u32) -> Radix<u32>, base: u32) {
    const N: u32 = 1234;
    let radix = Radix::new(N, base);

    format!("{}", radix)
        .should()
        .be_equal_to(format!("{}", f(N)));
}

#[fact]
fn alternate_capitalize() {
    let r = Radix::new(100, 28);

    format!("{:#}", r).should().be_equal_to("3G");
}
