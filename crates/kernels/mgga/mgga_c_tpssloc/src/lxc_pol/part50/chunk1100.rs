//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1100/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1100<F: Float>(t1409: F, t607: F, t8307: F, t8513: F, t31011: F, t3966: F, t32: F, t2240: F, t8308: F, t33114: F, t645: F, t113862: F, t113869: F, t113874: F, t113880: F, t113883: F, t113888: F, t119880: F, t119884: F, t119888: F, t119892: F, t119897: F, t119902: F, t119905: F, t119909: F, t119913: F, t119917: F, t31004: F, t31010: F, t31013: F, t33111: F, t8304: F) -> (F,) {
    let t119924 = t8513 * t8307 * t607 * t1409;
    let t119928 = t8513 * t31011 * t3966;
    let t119931 = t32 * t607;
    let t119932 = t2240 * t119931;
    let t119933 = t8308 * t1409;
    let t119938 = t8513 * t33114 * t645;
    let t119941 = 5.0 / 6.0 * t113862 * t119880 + 5.0 / 6.0 * t113862 * t119884 - 5.0 / 18.0 * t113869 * t119888 - 5.0 / 18.0 * t113874 * t119892 - 5.0 / 18.0 * t113869 * t119897 - 5.0 / 18.0 * t113874 * t119902 - 5.0 / 36.0 * t119905 * t31013 + 35.0 / 24.0 * t113883 * t119909 - 5.0 / 12.0 * t113888 * t119913 - 5.0 / 12.0 * t31004 * t119917 - 5.0 / 36.0 * t113880 * t33111 - 5.0 / 36.0 * t31010 * t119924 - 5.0 / 36.0 * t31010 * t119928 + 5.0 / 18.0 * t119932 * t8304 * t119933 - 5.0 / 12.0 * t113888 * t119938;
    (t119941,)
}
