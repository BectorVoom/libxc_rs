//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1415/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1415<F: Float>(t22561: F, t4034: F, t3652: F, t652: F, t6534: F, t1874: F, t45602: F, t6525: F, t9348: F, t15904: F, t22574: F, t31035: F) -> (F, F, F, F, F) {
    let t83674 = F::cast_from(12.0_f64) * t4034 * t22561;
    let t83677 = F::cast_from(6.0_f64) * t652 * t3652 * t6534;
    let t83679 = F::cast_from(6.0_f64) * t45602 * t1874;
    let t83681 = F::cast_from(6.0_f64) * t9348 * t6525;
    let t83684 = F::cast_from(18.0_f64) * t22574 * t31035 * t15904;
    (t83674, t83677, t83679, t83681, t83684)
}
