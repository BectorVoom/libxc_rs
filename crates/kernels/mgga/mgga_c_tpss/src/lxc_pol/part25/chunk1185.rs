//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1185/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1185<F: Float>(t18444: F, t339: F, t4419: F, t790: F, t1246: F, t136: F, t1693: F, t19468: F, t19470: F, t5543: F, t236: F, t60698: F, t18464: F, t4480: F, t1642: F, t60706: F) -> (F, F, F, F, F, F) {
    let t65592 = t339 * t18444 * t790 * t4419;
    let t65595 = t1693 * t1246 * t136;
    let t65600 = t5543 * t19468 * t19470;
    let t65607 = t339 * t60698 * t236;
    let t65616 = t18464 * t4480;
    let t65624 = t60706 * t1642;
    (t65592, t65595, t65600, t65607, t65616, t65624)
}
