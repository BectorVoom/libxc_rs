//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 786/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk786<F: Float>(t2274: F, t50: F, t2244: F, t2250: F, t22510: F, t7251: F, t67: F, t1864: F, t6509: F, t7255: F, t2109: F, t22489: F) -> (F, F, F, F) {
    let t24498 = t50 * t2274;
    let t24503 = F::new(5.0) / F::new(18.0) * t24498 * t2244 - F::new(5.0) / F::new(6.0) * t7251 * t2250 - t22510;
    let t24504 = t24503 * t67;
    let t24505 = t24504 * t1864;
    let t24508 = t7255 * t6509;
    let t24511 = t2109 * t22489;
    (t24503, t24505, t24508, t24511)
}
