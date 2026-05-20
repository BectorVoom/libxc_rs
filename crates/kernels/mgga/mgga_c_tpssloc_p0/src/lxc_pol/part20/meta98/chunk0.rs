//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 666/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk666<F: Float>(t40: F, t2244: F, t2250: F, t2433: F, t73: F, t197: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t2439 = piecewise3::<F>(t146, F::new(0.0), F::new(4.0) / F::new(9.0) * t2433 * t2244 + F::new(4.0) / F::new(3.0) * t73 * t2250);
    let t2440 = F::new(1.0) / t197;
    (t2439, t2440)
}
