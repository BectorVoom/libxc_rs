//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 948/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk948<F: Float>(t2022: F, t7961: F, t118573: F, t118586: F, t118588: F, t118596: F, t118602: F, t120350: F, t120363: F, t120375: F, t120393: F, t120416: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t122864 = t2022 * t7961;
    let t123566 = F::new(0.32298204875312312682e-2) * t118573;
    let t123571 = F::new(0.5383034145885385447e-3) * t118586;
    let t123572 = F::new(7.0) / F::new(144.0) * t118588;
    let t123576 = F::new(7.0) / F::new(576.0) * t118596;
    let t123578 = F::new(7.0) / F::new(576.0) * t118602;
    let t124139 = F::new(7.0) / F::new(576.0) * t120350;
    let t124142 = F::new(0.5383034145885385447e-3) * t120363;
    let t124146 = F::new(7.0) / F::new(144.0) * t120375;
    let t124154 = F::new(0.32298204875312312682e-2) * t120393;
    let t124163 = F::new(7.0) / F::new(576.0) * t120416;
    (t122864, t123566, t123571, t123572, t123576, t123578, t124139, t124142, t124146, t124154, t124163)
}
