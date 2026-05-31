//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1238/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1238<F: Float>(t7230: F, t7467: F, t16524: F, t8657: F, t33185: F, t1873: F, t7801: F, t3941: F, t2039: F, t12571: F, t8662: F, t7973: F, t8301: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33645 = F::cast_from(0.135e2_f64) * t7230 * t7467;
    let t33653 = F::cast_from(27.0_f64) * t16524 * t8657;
    let t33655 = F::cast_from(27.0_f64) * t33185 * t8657;
    let t33656 = t7801 * t1873;
    let t33658 = F::cast_from(27.0_f64) * t3941 * t33656;
    let t33659 = t2039 * t7467;
    let t33661 = F::cast_from(27.0_f64) * t3941 * t33659;
    let t33669 = t12571 * t8662;
    let t33676 = t8301 * t7973;
    (t33645, t33653, t33655, t33656, t33658, t33659, t33661, t33669, t33676)
}
