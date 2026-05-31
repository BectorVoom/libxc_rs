//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1116/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1116<F: Float>(t7754: F, t8450: F, t31047: F, t7687: F, t1983: F, t191: F, t192: F, t7681: F, t2020: F, t3701: F, t7752: F, t2019: F) -> (F, F, F, F, F, F, F) {
    let t33127 = t8450 * t7754;
    let t33129 = t31047 * t7687;
    let t33131 = F::cast_from(3.0_f64) * t1983 * t33129;
    let t33133 = t7681 * t191 * t192;
    let t33134 = t33133 * t2020;
    let t33136 = t3701 * t7752;
    let t33137 = t2019 * t33136;
    (t33127, t33129, t33131, t33133, t33134, t33136, t33137)
}
