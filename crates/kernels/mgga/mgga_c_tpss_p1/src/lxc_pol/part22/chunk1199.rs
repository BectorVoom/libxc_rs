//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1199/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1199<F: Float>(t1215: F, t18436: F, t1693: F, t527: F, t3247: F, t3251: F, t5716: F, t3255: F, t64: F, t234: F, t339: F, t3263: F) -> (F, F, F, F, F, F) {
    let t18437 = t18436 * t1215;
    let t18438 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t18437;
    let t18439 = t1693 * t527;
    let t18440 = t18439 * t3247;
    let t18442 = t5716 * t3251;
    let t18444 = t3255 * t64;
    let t18446 = t339 * t18444 * t234;
    let t18447 = t18446 * t3263;
    (t18437, t18438, t18440, t18442, t18444, t18447)
}
