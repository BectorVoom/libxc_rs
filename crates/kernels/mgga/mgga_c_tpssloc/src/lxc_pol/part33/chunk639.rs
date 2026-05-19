//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 639/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk639<F: Float>(t3297: F, t5971: F, t136: F, t1113: F, t5975: F, t5979: F, t3282: F, t3294: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F) -> (F, F, F, F, F, F, F) {
    let t6011 = t3297 * t5971;
    let t6012 = t136 * t6011;
    let t6014 = t1113 * t5975;
    let t6015 = t136 * t6014;
    let t6017 = t1113 * t5979;
    let t6018 = t136 * t6017;
    let t6020 = -F::new(0.9494625e0) * t5993 + F::new(0.1898925e1) * t6000 + t3282 - F::cast_from(0.19931111111111111111e0_f64) * t4721 - F::cast_from(0.19931111111111111111e0_f64) * t5973 + F::cast_from(0.59793333333333333334e0_f64) * t5977 + F::cast_from(0.29896666666666666667e0_f64) * t5981 + F::new(0.15358125e0) * t6006 + F::new(0.3071625e0) * t6008 + t3294 - F::cast_from(0.10954222222222222222e0_f64) * t4770 - F::cast_from(0.27385555555555555556e-1_f64) * t6012 + F::cast_from(0.16431333333333333333e0_f64) * t6015 + F::cast_from(0.82156666666666666667e-1_f64) * t6018;
    (t6011, t6012, t6014, t6015, t6017, t6018, t6020)
}
