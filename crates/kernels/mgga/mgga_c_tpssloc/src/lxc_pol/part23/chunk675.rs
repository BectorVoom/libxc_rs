//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 675/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk675<F: Float>(t349: F, t5914: F, t1634: F, t3174: F, t381: F, t5872: F, t3188: F, t1615: F, t1625: F, t1060: F, t5866: F, t3201: F, t383: F, t1058: F, t1610: F, t1630: F, t1632: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5915 = t349 * t5914;
    let t5919 = t1634 * t1634;
    let t5920 = t3174 * t5919;
    let t5928 = t381 * t5872;
    let t5929 = t5928 * t3188;
    let t5932 = t1625 * t1615;
    let t5933 = t5932 * t1060;
    let t5936 = t381 * t5866;
    let t5937 = t5936 * t1060;
    let t5939 = t5928 * t3201;
    let t5941 = t383 * t5914;
    let t5943 = 2.0 * t1058 * t5933 + t1058 * t5937 + 2.0 * t1610 * t1632 + 2.0 * t1630 * t4669 + 2.0 * t3186 * t5929 - t3200 * t5939 + t353 * t5941 + t384 * t5903;
    (t5915, t5919, t5920, t5928, t5929, t5933, t5936, t5937, t5939, t5941, t5943)
}
