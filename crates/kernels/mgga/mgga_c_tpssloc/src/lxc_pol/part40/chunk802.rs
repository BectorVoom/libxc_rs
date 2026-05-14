//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 802/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk802<F: Float>(t2423: F, t3686: F, t3690: F, t3695: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t6300: F, t6322: F, t225: F, t6401: F, t3843: F, t6330: F) -> (F, F) {
    let t6402 = -t3690 - t3695 + t6322 + t3686 + t3819 + t3821 + t3823 - t2423 - t6300 + t3825 - t3832 - t3836;
    let t6404 = (t6401 + t6402) * t225;
    let t6408 = t3843 * t6330;
    (t6404, t6408)
}
