//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 982/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk982<F: Float>(t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F) -> (F, F) {
    let t3660 = F::new(2.0) * t1268 * t2363 + F::new(4.0) * t2314 * t671 + F::new(2.0) * t2319 * t88 + t2312;
    let t3664 = F::new(1.0) / t526;
    (t3660, t3664)
}
