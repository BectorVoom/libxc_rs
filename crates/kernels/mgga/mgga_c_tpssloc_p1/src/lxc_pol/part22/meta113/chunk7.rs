//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 772/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk772<F: Float>(t3236: F, t407: F) -> (F, F, F) {
    let t3274 = F::new(4.0) / F::new(9.0) * t3236;
    let t3282 = F::cast_from(0.39862222222222222223e0_f64) * t3236;
    let t3287 = F::new(1.0)/F::sqrt(t407);
    (t3274, t3282, t3287)
}
