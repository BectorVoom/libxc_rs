//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1348/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1348<F: Float>(t23168: F, t23223: F, t1882: F, t81686: F, t9537: F, t1880: F, t23218: F, t23237: F, t213: F, t225: F, t852: F, t22986: F, t23272: F) -> (F, F, F, F) {
    let t82150 = t23168 * t23223;
    let t82153 = t81686 * t9537 * t1882;
    let t82154 = F::new(0.13707783890401886971e-2) * t82153;
    let t82156 = t1880 * t23237 * t23218;
    let t82159 = t213 * t852 * t225;
    let t82161 = t22986 * t82159 * t23272;
    (t82150, t82154, t82156, t82161)
}
