//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 785/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk785<F: Float>(t25111: F, t815: F, t23097: F, t13223: F, t232: F, t6605: F, t23077: F, t6604: F, t4255: F, t841: F, t4234: F, t23083: F, t7500: F) -> (F, F, F, F, F, F, F) {
    let t25112 = t815 * t25111;
    let t25113 = t23097 * t25112;
    let t25115 = t13223 * t232;
    let t25116 = t815 * t25115;
    let t25117 = t6605 * t25116;
    let t25119 = t23077 * t6604;
    let t25120 = t841 * t4255;
    let t25121 = t25119 * t25120;
    let t25123 = t815 * t4234;
    let t25124 = t6605 * t25123;
    let t25126 = t23083 * t7500;
    (t25113, t25115, t25117, t25119, t25121, t25124, t25126)
}
