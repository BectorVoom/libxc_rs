//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 933/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk933<F: Float>(t1358: F, t6379: F, t12211: F, t6371: F, t3726: F, t6375: F, t12385: F, t6390: F, t16288: F, t1827: F, t1340: F, t19815: F) -> (F, F, F, F, F, F) {
    let t19834 = t6379 * t1358;
    let t19839 = t12211 * t6371;
    let t19841 = t3726 * t6375;
    let t19851 = t12385 * t6390;
    let t19853 = t16288 * t1827;
    let t19855 = t19815 * t1340;
    (t19834, t19839, t19841, t19851, t19853, t19855)
}
