//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1186/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1186<F: Float>(t4034: F, t8533: F, t1873: F, t7156: F, t652: F, t1388: F, t2018: F, t26558: F, t26161: F, t24462: F, t24465: F, t7015: F) -> (F, F, F, F, F, F, F, F) {
    let t31771 = F::new(2.0) * t4034 * t8533;
    let t31772 = t7156 * t1873;
    let t31774 = F::new(2.0) * t652 * t31772;
    let t31775 = t2018 * t1388;
    let t31776 = t26558 * t31775;
    let t31778 = F::new(2.0) * t26161 * t31776;
    let t31799 = F::new(0.135e2) * t24462 * t1873;
    let t31801 = F::new(27.0) * t24465 * t7015;
    (t31771, t31772, t31774, t31775, t31776, t31778, t31799, t31801)
}
