//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 892/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk892<F: Float>(t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F, t423: F) -> F {
    let t11459 = F::new(0.55403703703703703703e-1) * t11135;
    let t11470 = -t11459 + F::new(0.23744444444444444444e-1) * t11137 + F::new(0.11872222222222222222e-1) * t11139 - F::new(0.35616666666666666666e-1) * t11141 - F::new(0.17808333333333333333e-1) * t11143 + F::new(0.19787037037037037037e-1) * t11150 - F::new(0.71233333333333333332e-1) * t11156 - F::new(0.35616666666666666666e-1) * t11161 + F::new(0.10685e0) * t11165 + F::new(0.10685e0) * t11170 + F::new(0.17808333333333333333e-1) * t11174;
    let t11472 = F::new(0.621814e-1) * t11470 * t423;
    t11472
}
