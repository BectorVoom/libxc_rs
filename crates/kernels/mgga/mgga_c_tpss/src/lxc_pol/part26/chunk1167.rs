//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1167/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1167<F: Float>(t1630: F, t18436: F, t136: F, t527: F, t1693: F, t215: F, t4478: F, t4409: F, t5716: F, t18444: F, t236: F, t339: F) -> (F, F, F, F, F, F, F) {
    let t19466 = t18436 * t1630;
    let t19468 = t527 * t136;
    let t19469 = t1693 * t19468;
    let t19470 = t215 * t4478;
    let t19471 = t19469 * t19470;
    let t19473 = t5716 * t4409;
    let t19476 = t339 * t18444 * t236;
    (t19466, t19468, t19469, t19470, t19471, t19473, t19476)
}
