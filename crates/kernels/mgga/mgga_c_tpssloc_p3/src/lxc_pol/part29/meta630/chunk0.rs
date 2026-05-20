//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2077/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2077<F: Float>(t12813: F, t1873: F, t3941: F, t55341: F, t12524: F, t26542: F, t22479: F, t5371: F, t66940: F, t7769: F, t55353: F, t7015: F) -> (F, F, F, F, F, F) {
    let t86625 = F::new(27.0) * t3941 * t1873 * t12813;
    let t86629 = F::new(0.135e2) * t55341 * t1873;
    let t86631 = F::new(54.0) * t12524 * t26542;
    let t86633 = F::new(0.135e2) * t5371 * t22479;
    let t86635 = F::new(54.0) * t66940 * t7769;
    let t86637 = F::new(54.0) * t55353 * t7015;
    (t86625, t86629, t86631, t86633, t86635, t86637)
}
