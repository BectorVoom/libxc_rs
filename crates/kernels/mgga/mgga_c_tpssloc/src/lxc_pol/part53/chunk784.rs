//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 784/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk784<F: Float>(t1307: F, t31549: F, t22635: F, t22633: F, t2091: F, t3886: F) -> (F, F, F, F) {
    let t31550 = t31549 * t1307;
    let t31551 = t22635 * t31550;
    let t31552 = t22633 * t31551;
    let t31558 = t3886 * t2091;
    (t31550, t31551, t31552, t31558)
}
