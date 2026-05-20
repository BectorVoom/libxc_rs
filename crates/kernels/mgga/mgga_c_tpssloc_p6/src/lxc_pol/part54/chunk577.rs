//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 577/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk577<F: Float>(t2826: F, t4338: F, t136: F, t4343: F, t908: F, t4347: F, t2766: F, t2810: F, t2823: F, t2824: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F) -> (F, F, F, F) {
    let t4386 = t2826 * t4338;
    let t4387 = t136 * t4386;
    let t4389 = t908 * t4343;
    let t4390 = t136 * t4389;
    let t4392 = t908 * t4347;
    let t4393 = t136 * t4392;
    let t4395 = -F::new(0.9494625e0) * t4363 + F::new(0.1898925e1) * t4371 + t2810 + F::cast_from(0.99655555555555555557e-1_f64) * t2766 + F::cast_from(0.99655555555555555557e-1_f64) * t4335 - F::cast_from(0.19931111111111111111e0_f64) * t4340 + F::cast_from(0.59793333333333333334e0_f64) * t4345 - F::cast_from(0.29896666666666666667e0_f64) * t4349 + F::new(0.15358125e0) * t4379 + F::new(0.3071625e0) * t4381 + t2823 + F::cast_from(0.54771111111111111111e-1_f64) * t2824 + F::cast_from(0.54771111111111111111e-1_f64) * t4384 - F::cast_from(0.27385555555555555556e-1_f64) * t4387 + F::cast_from(0.16431333333333333333e0_f64) * t4390 - F::cast_from(0.82156666666666666667e-1_f64) * t4393;
    (t4387, t4390, t4393, t4395)
}
