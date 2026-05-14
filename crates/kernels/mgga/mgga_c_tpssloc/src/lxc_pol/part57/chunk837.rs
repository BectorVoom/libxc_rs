//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 837/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk837<F: Float>(t33281: F, t6914: F, t1338: F, t33266: F, t33285: F, t6883: F, t33284: F, t6897: F, t794: F, t22892: F, t22893: F, t33276: F, t22751: F, t33277: F, t552: F, t7918: F) -> (F, F, F, F, F, F, F) {
    let t122462 = t6914 * t33281;
    let t122475 = t1338 * t33266;
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    let t122533 = t22892 * t22893 * t33276;
    let t122535 = t22751 * t33277;
    let t122537 = t552 * t7918;
    (t122462, t122475, t122503, t122507, t122533, t122535, t122537)
}
