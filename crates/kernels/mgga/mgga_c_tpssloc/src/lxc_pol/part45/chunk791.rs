//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 791/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk791<F: Float>(t23604: F, t3187: F, t23603: F, t3192: F, t6800: F, t6799: F, t225: F, t6733: F, t6786: F, t1949: F, t2966: F, t1920: F) -> (F, F, F, F) {
    let t23605 = t3187 * t23604;
    let t23606 = t23603 * t23605;
    let t23609 = t3192 * t6800;
    let t23610 = t6799 * t23609;
    let t23613 = t6733 * t225;
    let t23614 = t23613 * t6786;
    let t23617 = t2966 * t1949;
    let t23619 = F::cast_from(0.18277045187202515961e-2_f64) * t1920 * t23617;
    (t23606, t23610, t23614, t23619)
}
