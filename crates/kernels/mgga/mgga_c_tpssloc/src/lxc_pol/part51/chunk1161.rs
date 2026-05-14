//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1161/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1161<F: Float>(t22642: F, t22643: F, t8621: F, t22716: F, t8612: F, t22674: F, t31607: F, t6897: F, t31550: F, t81228: F, t81326: F, t31551: F, t81159: F, t115352: F, t6907: F, t3886: F, t7213: F) -> (F, F, F, F, F, F, F) {
    let t115550 = t22642 * t22643 * t8621;
    let t115551 = 0.82246703342411321824e-2 * t115550;
    let t115566 = t22716 * t8612;
    let t115567 = 0.63969658155208805863e-1 * t115566;
    let t115572 = t6897 * t22674 * t31607;
    let t115586 = t81228 * t81326 * t31550;
    let t115596 = t81159 * t31551;
    let t115601 = t6897 * t115352 * t6907;
    let t115614 = t3886 * t7213;
    (t115551, t115567, t115572, t115586, t115596, t115601, t115614)
}
