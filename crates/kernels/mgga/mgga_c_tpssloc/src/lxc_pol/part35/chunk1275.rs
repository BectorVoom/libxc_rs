//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1275/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1275<F: Float>(t5456: F, t7982: F, t105213: F, t106733: F, t106736: F, t106738: F, t106741: F, t106744: F, t1442: F, t19451: F, t20296: F, t20717: F, t20720: F, t2114: F, t2165: F, t22425: F, t27863: F, t29848: F, t33690: F, t510: F, t5450: F, t5457: F, t5493: F, t5494: F, t6287: F, t6468: F, t652: F, t7266: F, t7983: F, t7989: F, t8103: F, t8107: F) -> (F, F) {
    let t108902 = t7982 * t5456;
    let t108918 = -6.0 * t5493 * t652 * t8103 - 6.0 * t108902 * t510 - 3.0 * t1442 * t29848 - 6.0 * t19451 * t7989 - 6.0 * t20296 * t2165 - 6.0 * t20717 * t7266 - 2.0 * t20720 * t7266 - t2114 * t22425 - 6.0 * t27863 * t5494 - 6.0 * t33690 * t5494 - 3.0 * t5450 * t8103 - 6.0 * t5457 * t8103 - 3.0 * t6287 * t7983 + 3.0 * t6468 * t8107 + t105213 - t106733 - t106736 - t106738 + t106741 - t106744;
    (t108902, t108918)
}
