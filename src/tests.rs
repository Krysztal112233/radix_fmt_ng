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
#[case(37, "2q")]
#[case(38, "2o")]
#[case(39, "2m")]
#[case(40, "2k")]
#[case(41, "2i")]
#[case(42, "2g")]
#[case(43, "2e")]
#[case(44, "2c")]
#[case(45, "2a")]
#[case(46, "28")]
#[case(47, "26")]
#[case(48, "24")]
#[case(49, "22")]
#[case(50, "20")]
#[case(51, "1N")]
#[case(52, "1M")]
#[case(53, "1L")]
#[case(54, "1K")]
#[case(55, "1J")]
#[case(56, "1I")]
#[case(57, "1H")]
#[case(58, "1G")]
#[case(59, "1F")]
#[case(60, "1E")]
#[case(61, "1D")]
fn base_formatting(base: u8, hundred_formatted: &str) {
    let zero = Radix::new(0, base);
    let hundred = Radix::new(100, base);

    format!("{}", zero).should().be_equal_to("0");
    format!("{}", hundred)
        .should()
        .be_equal_to(hundred_formatted);
}

#[fact]
fn max_size_is_ok() {
    let base_3 =
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010";

    format!("{}", radix(u128::MAX, 3))
        .should()
        .be_equal_to(base_3);
}

#[fact]
fn binary_fallback_is_ok() {
    let base_2: String = std::iter::repeat_n('1', 128).collect();

    format!("{}", radix(-1_i128, 2))
        .should()
        .be_equal_to(base_2);
}

#[fact]
fn same_number_as_base() {
    format!("{}", radix(9, 9)).should().be_equal_to("10");
}

#[fact]
fn whatever_number() {
    format!("{}", radix(5, 9)).should().be_equal_to("5");
}

mod types {
    use crate::*;
    use fluid::prelude::*;

    const U: &str = "10201";

    #[fact]
    fn u8_is_ok() {
        let n: u8 = 100;

        format!("{}", radix(n, 3)).should().be_equal_to(U);
    }

    #[fact]
    fn i8_is_ok() {
        let n: i8 = -100;
        let u = n as u8;

        format!("{}", radix(n, 3))
            .should()
            .be_equal_to(format!("{}", radix(u, 3)));
    }

    #[fact]
    fn u16_is_ok() {
        let n: u16 = 100;

        format!("{}", radix(n, 3)).should().be_equal_to(U);
    }

    #[fact]
    fn i16_is_ok() {
        let n: i16 = -100;
        let u = n as u16;

        format!("{}", radix(n, 3))
            .should()
            .be_equal_to(format!("{}", radix(u, 3)));
    }

    #[fact]
    fn u32_is_ok() {
        let n: u32 = 100;

        format!("{}", radix(n, 3)).should().be_equal_to(U);
    }

    #[fact]
    fn i32_is_ok() {
        let n: i32 = -100;
        let u = n as u32;

        format!("{}", radix(n, 3))
            .should()
            .be_equal_to(format!("{}", radix(u, 3)));
    }

    #[fact]
    fn u64_is_ok() {
        let n: u64 = 100;

        format!("{}", radix(n, 3)).should().be_equal_to(U);
    }

    #[fact]
    fn i64_is_ok() {
        let n: i64 = -100;
        let u = n as u64;

        format!("{}", radix(n, 3))
            .should()
            .be_equal_to(format!("{}", radix(u, 3)));
    }

    #[fact]
    fn usize_is_ok() {
        let n: usize = 100;

        format!("{}", radix(n, 3)).should().be_equal_to(U);
    }

    #[fact]
    fn isize_is_ok() {
        let n: isize = -100;
        let u = n as usize;

        format!("{}", radix(n, 3))
            .should()
            .be_equal_to(format!("{}", radix(u, 3)));
    }
}
