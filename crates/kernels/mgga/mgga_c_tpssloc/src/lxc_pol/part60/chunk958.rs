//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 958/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk958<F: Float>(t12571: F, t31863: F, t45844: F, t8662: F, t33676: F, t9239: F, t118573: F, t118586: F, t118588: F, t118596: F, t118602: F, t120350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122976 = t12571 * t31863;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123566 = F::cast_from(0.32298204875312312682e-2_f64) * t118573;
    let t123571 = F::cast_from(0.5383034145885385447e-3_f64) * t118586;
    let t123572 = F::new(7.0) / F::new(144.0) * t118588;
    let t123576 = F::new(7.0) / F::new(576.0) * t118596;
    let t123578 = F::new(7.0) / F::new(576.0) * t118602;
    let t124139 = F::new(7.0) / F::new(576.0) * t120350;
    (t122976, t122988, t123001, t123566, t123571, t123572, t123576, t123578, t124139)
}
