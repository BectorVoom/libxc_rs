//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2310/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310<F: Float>(t16606: F, t17120: F, t1877: F, t40764: F, t40766: F, t4255: F, t4303: F, t4314: F, t46292: F, t67176: F, t67178: F, t67180: F, t67183: F, t67186: F, t67191: F) -> F {
    let t67195 = F::new(18.0) * t16606 * t4255 * t4314 + F::new(6.0) * t17120 * t1877 * t4303 + t40764 + t40766 + t46292 - t67176 + t67178 + t67180 + t67183 + t67186 + t67191;
    t67195
}
