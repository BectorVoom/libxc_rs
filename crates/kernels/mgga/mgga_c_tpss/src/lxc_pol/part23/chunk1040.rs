//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1040/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1040<F: Float>(t1474: F, t2723: F, t2703: F, t11707: F, t11733: F, t11743: F, t11750: F, t11753: F, t11760: F, t11767: F, t11771: F, t1477: F, t220: F, t2782: F, t2786: F, t2798: F, t2799: F, t368: F, t3987: F, t3997: F, t4001: F, t4004: F, t4008: F, t9077: F, t9089: F, t9094: F, t9117: F, t948: F, t983: F, t985: F) -> (F,) {
    let t11774 = t1474 * t2723;
    let t11782 = t1474 * t2703;
    let t11789 = 2.0 * t3987 * t948 * t983 * t985 + t11707 * t220 * t368 + 6.0 * t11733 * t1477 * t9077 - 6.0 * t11743 * t1477 * t9094 + t11750 * t983 * t985 + 2.0 * t11753 * t983 * t985 + t11760 * t983 * t985 - t11767 * t1477 * t2798 + t11771 * t1477 * t9117 + 2.0 * t11774 * t2782 * t2786 - t11774 * t2798 * t2799 + t11782 * t983 * t985 + 2.0 * t1477 * t2782 * t9089 + 4.0 * t2782 * t3997 * t4001 + 4.0 * t2782 * t3997 * t4004 - 2.0 * t2798 * t4001 * t4008 - 2.0 * t2798 * t4004 * t4008;
    (t11789,)
}
