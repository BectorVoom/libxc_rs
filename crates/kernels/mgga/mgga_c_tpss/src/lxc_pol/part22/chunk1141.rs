//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1141/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1141<F: Float>(t19817: F, t19818: F, t1398: F, t580: F, t30: F, t3724: F, t1288: F, t750: F, t821: F, t33: F, t823: F, t3683: F, t14076: F, t18246: F, t1006: F, t1364: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19819 = t19817 * t19818;
    let t19821 = t580 * t1398;
    let t19825 = t30 * t3724;
    let t19829 = t1288 * t750;
    let t19836 = t1288 * t821;
    let t20011 = t823 * t33;
    let t20012 = t20011 * t3683;
    let t20018 = t18246 * t14076;
    let t20021 = t1006 * t1364;
    (t19819, t19821, t19825, t19829, t19836, t20011, t20012, t20018, t20021)
}
