//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2638/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2638<F: Float>(t2585: F, t3732: F, t46853: F, t5308: F, t16118: F, t9577: F, t212: F, t5187: F, t12225: F, t2586: F, t16100: F, t782: F) -> (F, F, F, F, F) {
    let t54643 = t2585 * t3732 * t46853 * t5308;
    let t54663 = t9577 * t16118;
    let t54665 = t212 * t5187;
    let t54667 = t2586 * t12225 * t54665;
    let t54670 = t782 * t16100;
    (t54643, t54663, t54665, t54667, t54670)
}
