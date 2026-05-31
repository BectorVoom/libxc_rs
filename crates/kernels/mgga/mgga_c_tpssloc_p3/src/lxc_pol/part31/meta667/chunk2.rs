//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1962/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1962<F: Float>(t26756: F, t98069: F, t1877: F, t2219: F, t7845: F, t2752: F, t29105: F, t24191: F, t99053: F, t1408: F, t2057: F, t24339: F, t25028: F, t2522: F, t25381: F, t26563: F, t26740: F, t26744: F, t28456: F, t28462: F, t29106: F, t6542: F, t6671: F, t7114: F, t84800: F, t98012: F, t98020: F, t98086: F, t98112: F, t99060: F) -> (F, F, F, F, F) {
    let t101211 = F::cast_from(2.0_f64) * t26756 * t98069;
    let t101220 = F::cast_from(2.0_f64) * t1877 * t7845 * t2219;
    let t101226 = t29105 * t2752;
    let t101241 = F::cast_from(6.0_f64) * t24191 * t99053;
    let t101248 = -t101211 + F::cast_from(3.0_f64) * t2522 * t2057 * t98020 - t1877 * t24339 * t28462 / F::cast_from(2.0_f64) + t101220 + t1877 * t84800 * t28456 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t29106 * t6542 - t1877 * t101226 * t6671 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t7845 * t25028 - t1877 * t7114 * t98086 / F::cast_from(2.0_f64) - t1877 * t26744 * t25381 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t98012 + t101241 + t1877 * t26740 * t1408 + F::cast_from(6.0_f64) * t26563 * t99060 + F::cast_from(6.0_f64) * t24191 * t98112;
    (t101211, t101220, t101226, t101241, t101248)
}
