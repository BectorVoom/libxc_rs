//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1140/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1140<F: Float>(t25010: F, t8450: F, t31051: F, t7458: F, t2314: F, t32663: F, t4034: F, t1873: F, t25958: F, t652: F, t1874: F, t96361: F, t24999: F, t6525: F, t12725: F, t8323: F) -> (F, F, F, F, F, F, F, F) {
    let t120738 = t8450 * t25010;
    let t120740 = t7458 * t31051;
    let t120742 = t2314 * t32663;
    let t120744 = t4034 * t32663;
    let t120747 = t652 * t25958 * t1873;
    let t120749 = t96361 * t1874;
    let t120751 = t24999 * t6525;
    let t120753 = t12725 * t8323;
    (t120738, t120740, t120742, t120744, t120747, t120749, t120751, t120753)
}
