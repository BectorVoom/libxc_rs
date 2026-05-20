//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1245/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1245<F: Float>(t1873: F, t90400: F, t120112: F, t112594: F, t119815: F, t119820: F, t119996: F, t120140: F, t120141: F, t120143: F, t120146: F, t120149: F, t120151: F, t120153: F, t1458: F, t31224: F, t4072: F, t671: F) -> F {
    let t120163 = t90400 * t1873;
    let t120165 = F::new(2.0) * t120112;
    let t120166 = F::new(2.0) * t112594 * t1458 + F::new(2.0) * t119815 * t671 + F::new(2.0) * t119820 * t1458 + F::new(2.0) * t31224 * t4072 + t119996 + t120140 + F::new(4.0) * t120141 + F::new(4.0) * t120143 + F::new(4.0) * t120146 + F::new(4.0) * t120149 + F::new(4.0) * t120151 + F::new(4.0) * t120153 + F::new(4.0) * t120163 + t120165;
    t120166
}
