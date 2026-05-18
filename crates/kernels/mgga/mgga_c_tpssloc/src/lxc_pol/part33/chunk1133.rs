//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1133/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1133<F: Float>(t254: F, t382: F, t10164: F, t1955: F, t225: F, t7569: F, t1921: F, t25749: F, t7561: F, t968: F, t1920: F, t1625: F, t6688: F) -> (F, F, F, F, F, F) {
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    let t25778 = t7569 * t225;
    let t25784 = t1921 * t25749;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    (t25757, t25758, t25778, t25784, t25807, t25810)
}
