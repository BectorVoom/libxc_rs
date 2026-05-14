//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 922/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk922<F: Float>(t2143: F, t3622: F, t10552: F, t124: F, t762: F, t1369: F, t8176: F, t1368: F, t2116: F, t3618: F, t8167: F, t3621: F, t750: F, t2133: F, t2158: F, t339: F, t790: F) -> (F, F, F, F, F, F, F, F) {
    let t10630 = 7.0 / 72.0 * t2143 * t3622;
    let t10631 = t124 * t10552;
    let t10632 = t762 * t10631;
    let t10635 = t8176 * t1369;
    let t10638 = t762 * t1368 * t2116;
    let t10642 = 7.0 / 24.0 * t8167 * t3618;
    let t10644 = t762 * t3621 * t750;
    let t10648 = t762 * t1368 * t2133;
    let t10652 = t339 * t2158 * t790;
    (t10630, t10632, t10635, t10638, t10642, t10644, t10648, t10652)
}
