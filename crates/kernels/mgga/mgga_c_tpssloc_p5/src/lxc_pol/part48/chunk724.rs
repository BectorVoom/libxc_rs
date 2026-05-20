//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 724/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk724<F: Float>(t2379: F, t6638: F, t6637: F, t23035: F, t6612: F, t835: F, t812: F, t831: F, t2686: F, t6614: F, t2627: F, t59: F) -> (F, F, F, F, F) {
    let t23036 = t6638 * t2379;
    let t23037 = t6637 * t23036;
    let t23038 = t23035 * t23037;
    let t23040 = t6612 * t835;
    let t23041 = t812 * t23040;
    let t23042 = t23041 * t831;
    let t23043 = F::new(7.0) / F::new(1152.0) * t23042;
    let t23044 = t6614 * t2686;
    let t23046 = t2627 * t59;
    (t23038, t23042, t23043, t23044, t23046)
}
