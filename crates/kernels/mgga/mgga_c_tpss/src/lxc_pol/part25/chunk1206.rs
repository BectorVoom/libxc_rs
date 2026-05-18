//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1206/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1206<F: Float>(t19817: F, t19818: F, t1398: F, t580: F, t30: F, t3724: F, t1288: F, t750: F, t821: F, t33: F, t823: F, t3683: F) -> (F, F, F, F, F, F, F) {
    let t19819 = t19817 * t19818;
    let t19821 = t580 * t1398;
    let t19825 = t30 * t3724;
    let t19829 = t1288 * t750;
    let t19836 = t1288 * t821;
    let t20011 = t823 * t33;
    let t20012 = t20011 * t3683;
    (t19819, t19821, t19825, t19829, t19836, t20011, t20012)
}
