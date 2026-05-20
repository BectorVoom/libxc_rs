//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1847/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1847<F: Float>(t2113: F, t2319: F, t2363: F, t23844: F, t23846: F, t23848: F, t23850: F, t23852: F, t23854: F, t24543: F, t24932: F, t671: F, t7266: F) -> (F, F) {
    let t24935 = t2113 * t2319;
    let t24939 = F::new(2.0) * t2363 * t7266 + F::new(4.0) * t24932 * t671 + t23844 + t23846 + t23848 + t23850 + t23852 + t23854 + t24543 + F::new(2.0) * t24935;
    (t24935, t24939)
}
