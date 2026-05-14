//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1038/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1038<F: Float>(t114038: F, t1351: F, t22705: F, t22852: F, t550: F, t59: F, t22751: F, t31195: F, t22892: F, t22893: F, t31194: F, t22642: F, t22690: F, t31193: F, t552: F, t6955: F) -> (F, F, F, F, F, F) {
    let t114039 = 119.0 / 6912.0 * t114038;
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114057 = t22751 * t31195;
    let t114060 = t22892 * t22893 * t31194;
    let t114064 = 0.16449340668482264365e-1 * t22642 * t22690 * t31193;
    let t114069 = t552 * t6955;
    (t114039, t114046, t114057, t114060, t114064, t114069)
}
