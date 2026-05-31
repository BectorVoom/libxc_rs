//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1120/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1120<F: Float>(t32785: F, t33163: F, t3: F, t1873: F, t26523: F, t23880: F, t7769: F, t7010: F, t7467: F, t16524: F, t8319: F, t1458: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t33164 = t32785 + t33163;
    let t33165 = t3 * t33164;
    let t33177 = t26523 * t1873;
    let t33179 = t23880 * t7769;
    let t33181 = t7010 * t7467;
    let t33184 = F::cast_from(27.0_f64) * t16524 * t8319;
    let t33185 = t576 * t1458;
    (t33164, t33165, t33177, t33179, t33181, t33184, t33185)
}
