//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 356/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk356<F: Float>(t2223: F, t14: F, t21: F, t594: F, t598: F, t15: F) -> (F, F, F, F, F, F) {
    let t2224 = F::new(16.0) * t2223;
    let t2225 = t14 * t21;
    let t2226 = F::new(0.778e2) * t2225;
    let t2228 = F::new(0.16272e3) * t594 * t598;
    let t2229 = t15 * t15;
    let t2230 = F::new(1.0) / t2229;
    (t2224, t2225, t2226, t2228, t2229, t2230)
}
