//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1110/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1110<F: Float>(t18009: F, t18770: F, t5572: F, t5831: F, t818: F, t1805: F, t2425: F, t2161: F, t18021: F, t2162: F, t226: F, t782: F, t5577: F, t2364: F, t1708: F, t18750: F, t228: F) -> (F, F, F, F, F, F, F, F) {
    let t18771 = t18770 * t18009;
    let t18775 = t5572 * t5831 * t818;
    let t18779 = t5572 * t1805 * t2425;
    let t18782 = t1805 * t2161;
    let t18784 = t18021 * t18782 * t2162;
    let t18788 = t5831 * t782 * t226;
    let t18789 = t5577 * t18788;
    let t18794 = t5577 * t1805 * t2364 * t226;
    let t18797 = t5577 * t18782 * t226;
    let t18800 = t1708 * t228 * t18750;
    (t18771, t18775, t18779, t18784, t18789, t18794, t18797, t18800)
}
