//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 858/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk858<F: Float>(t33245: F, t6897: F, t794: F, t131: F, t2240: F, t27331: F, t12571: F, t31863: F, t45844: F, t8662: F, t33676: F, t9239: F, t118573: F, t118586: F, t118588: F, t118596: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122551 = t6897 * t794 * t33245;
    let t122945 = t2240 * t27331 * t131;
    let t122976 = t12571 * t31863;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123566 = 0.32298204875312312682e-2 * t118573;
    let t123571 = 0.5383034145885385447e-3 * t118586;
    let t123572 = 7.0 / 144.0 * t118588;
    let t123576 = 7.0 / 576.0 * t118596;
    (t122551, t122945, t122976, t122988, t123001, t123566, t123571, t123572, t123576)
}
