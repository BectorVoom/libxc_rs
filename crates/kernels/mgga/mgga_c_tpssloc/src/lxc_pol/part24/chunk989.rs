//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 989/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk989<F: Float>(t11136: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F, t449: F) -> F {
    let t11176 = -t11136 + F::new(0.12361111111111111111e-1) * t11137 + F::new(0.61805555555555555556e-2) * t11139 - F::new(0.18541666666666666667e-1) * t11141 - F::new(0.92708333333333333334e-2) * t11143 + F::new(0.10300925925925925926e-1) * t11150 - F::new(0.37083333333333333333e-1) * t11156 - F::new(0.18541666666666666666e-1) * t11161 + F::new(0.55625000000000000001e-1) * t11165 + F::new(0.55625000000000000001e-1) * t11170 + F::new(0.92708333333333333333e-2) * t11174;
    let t11177 = t11176 * t449;
    t11177
}
