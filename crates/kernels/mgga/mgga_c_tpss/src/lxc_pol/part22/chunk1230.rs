//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1230/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1230<F: Float>(t18499: F, t18967: F, t1265: F, t5740: F, t5918: F, t1838: F, t3384: F, t3259: F, t18511: F, t3260: F, t1232: F, t520: F) -> (F, F, F, F, F, F) {
    let t18968 = t18967 * t18499;
    let t18972 = t5740 * t5918 * t1265;
    let t18976 = t5740 * t1838 * t3384;
    let t18979 = t1838 * t3259;
    let t18981 = t18511 * t18979 * t3260;
    let t18985 = t5918 * t1232 * t520;
    (t18968, t18972, t18976, t18979, t18981, t18985)
}
