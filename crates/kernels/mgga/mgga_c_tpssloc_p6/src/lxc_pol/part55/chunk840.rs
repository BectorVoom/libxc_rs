//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 840/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk840<F: Float>(t3: F, t8919: F, t577: F, t8506: F, t8508: F, t8699: F, t192: F, t533: F, t1390: F, t2018: F, t2229: F, t2239: F, t601: F) -> (F, F, F, F, F, F, F) {
    let t8920 = t3 * t8919;
    let t8927 = F::cast_from(0.45e1_f64) * t8919 * t577 + F::cast_from(27.0_f64) * t8699 + t8506 + t8508;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    let t9222 = t2229 * t3;
    let t9223 = F::cast_from(1.0_f64) / t9222;
    let t9231 = t601 * t2239;
    (t8920, t8927, t8944, t8945, t9222, t9223, t9231)
}
