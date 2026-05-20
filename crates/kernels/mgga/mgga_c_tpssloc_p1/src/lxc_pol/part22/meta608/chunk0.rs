//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2134/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2134<F: Float>(t10470: F, t11058: F, t381: F, t1615: F, t6739: F, t11064: F, t3199: F, t49649: F, t11045: F, t10164: F, t1634: F, t11190: F, t1670: F) -> (F, F, F, F, F, F, F) {
    let t50508 = t10470 * t11058 * t381;
    let t50509 = t1615 * t6739;
    let t50516 = t10470 * t11064 * t381;
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50628 = t10164 * t1634;
    let t50819 = t11190 * t1670;
    (t50508, t50509, t50516, t50592, t50610, t50628, t50819)
}
