//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1299/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1299<F: Float>(t3540: F, t8049: F, t15734: F, t7345: F, t1244: F, t1742: F, t3068: F, t24720: F, t27710: F, t15501: F, t24727: F, t3500: F, sigma2: F) -> (F, F, F, F, F) {
    let t95520 = t8049 * t3540;
    let t95550 = t7345 * t15734;
    let t95566 = t1244 * sigma2 * t1742 * t3068;
    let t95588 = t27710 * t24720;
    let t95623 = t3500 * t24727 * t15501;
    (t95520, t95550, t95566, t95588, t95623)
}
