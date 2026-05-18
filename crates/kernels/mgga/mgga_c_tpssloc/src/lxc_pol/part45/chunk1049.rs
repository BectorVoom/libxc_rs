//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1049/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1049<F: Float>(t1983: F, t31669: F, t6999: F, t115824: F, t115914: F, t115942: F, t115946: F, t115948: F, t115959: F, t115965: F, t1869: F, t2040: F, t22461: F, t2314: F, t24167: F, t24169: F, t24428: F, t31734: F, t3652: F, t3929: F, t510: F, t6515: F, t7050: F, t7061: F, t7156: F, t8450: F, t8519: F, t8604: F, t90041: F) -> F {
    let t115968 = F::new(2.0) * t1983 * t31669 * t6999;
    let t115969 = -F::new(2.0) * t115824 * t510 - t115914 * t510 - t1869 * t24428 - F::new(4.0) * t2040 * t90041 - F::new(4.0) * t22461 * t7050 - F::new(4.0) * t22461 * t7061 - F::new(4.0) * t2314 * t31734 + t24167 * t8450 + F::new(2.0) * t24169 * t8450 - t3652 * t8519 + t3929 * t8604 - F::new(2.0) * t6515 * t7156 - t115942 - t115946 - t115948 + t115959 + t115965 - t115968;
    t115969
}
