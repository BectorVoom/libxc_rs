//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1563/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1563<F: Float>(t11328: F, t11343: F, t1137: F, t1127: F, t3355: F, t427: F) -> (F, F, F, F) {
    let t11344 = t11328 + t11343;
    let t11345 = t11344 * t1137;
    let t11349 = F::new(1.0) / t3355 / t1127;
    let t11350 = t427 * t11349;
    (t11344, t11345, t11349, t11350)
}
