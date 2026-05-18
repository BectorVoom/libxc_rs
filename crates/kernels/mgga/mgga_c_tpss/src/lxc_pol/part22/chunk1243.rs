//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1243/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1243<F: Float>(t1759: F, t19619: F, t3537: F, t93: F, t30: F, t823: F, t3683: F, t14076: F, t17930: F, t1364: F, t580: F, t3610: F) -> (F, F, F, F, F, F, F) {
    let t19620 = t1759 * t19619;
    let t19656 = t93 * t3537;
    let t19671 = t823 * t30;
    let t19672 = t19671 * t3683;
    let t19678 = t17930 * t14076;
    let t19681 = t580 * t1364;
    let t19685 = t30 * t3610;
    (t19620, t19656, t19671, t19672, t19678, t19681, t19685)
}
