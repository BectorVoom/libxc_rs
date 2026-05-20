//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1272/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1272<F: Float>(t1998: F, t7708: F, t6926: F, t1339: F, t1825: F, t6936: F, t1814: F, t2002: F, t559: F, t1827: F, t6945: F, t1831: F, t6952: F) -> (F, F, F, F, F, F, F, F) {
    let t7709 = t1998 * t7708;
    let t7710 = t6926 * t7709;
    let t7712 = t1339 * t1825;
    let t7713 = t6936 * t7712;
    let t7715 = t1814 * t2002;
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    (t7709, t7710, t7712, t7713, t7715, t7716, t7718, t7720)
}
