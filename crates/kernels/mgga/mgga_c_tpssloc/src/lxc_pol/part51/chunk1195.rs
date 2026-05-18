//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1195/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1195<F: Float>(t7057: F, t8526: F, t532: F, t8639: F, t6879: F, t1983: F, t2314: F, t8533: F, t4034: F, t1873: F, t7156: F, t652: F) -> (F, F, F, F, F, F, F, F) {
    let t31753 = F::new(2.0) * t8526 * t7057;
    let t31758 = t532 * t8639;
    let t31759 = t31758 * t6879;
    let t31761 = F::new(3.0) * t1983 * t31759;
    let t31769 = F::new(2.0) * t2314 * t8533;
    let t31771 = F::new(2.0) * t4034 * t8533;
    let t31772 = t7156 * t1873;
    let t31774 = F::new(2.0) * t652 * t31772;
    (t31753, t31758, t31759, t31761, t31769, t31771, t31772, t31774)
}
