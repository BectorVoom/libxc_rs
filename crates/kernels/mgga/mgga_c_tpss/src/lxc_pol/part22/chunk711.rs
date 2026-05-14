//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 711/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk711<F: Float>(t1415: F, t2487: F, t849: F, t2455: F, t2491: F, t3746: F, t3751: F, t3756: F, t3760: F, t847: F, t2504: F, t854: F, t1421: F, t673: F, t2515: F, t3749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3773 = t2487 * t1415;
    let t3774 = t3773 * t849;
    let t3781 = t2491 + t2455 / 9.0 + t3746 / 9.0 - 2.0 / 9.0 * t3751 + 2.0 / 3.0 * t3756 - t3760 / 3.0;
    let t3782 = t847 * t3781;
    let t3789 = t2504 * t1415;
    let t3790 = t3789 * t849;
    let t3792 = t854 * t3781;
    let t3795 = t673 * t1421;
    let t3797 = t2515 * t3749;
    (t3773, t3774, t3781, t3782, t3789, t3790, t3792, t3795, t3797)
}
