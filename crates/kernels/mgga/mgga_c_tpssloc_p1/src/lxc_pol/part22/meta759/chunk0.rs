//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2549/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2549<F: Float>(t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63893: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71391: F) -> F {
    let t71571 = F::cast_from(0.99655555555555555555e0_f64) * t71124 - F::cast_from(0.26574814814814814815e0_f64) * t63332 + F::cast_from(0.39862222222222222223e0_f64) * t63334 - F::cast_from(0.29896666666666666667e0_f64) * t63336 - F::new(0.35876e1) * t71130 - F::cast_from(0.16431333333333333333e0_f64) * t63886 - F::cast_from(0.91285185185185185184e-1_f64) * t63888 + F::cast_from(0.5477111111111111111e0_f64) * t63893 + F::new(0.3071625e0) * t71391 + F::cast_from(0.39862222222222222223e1_f64) * t71135 - F::cast_from(0.19931111111111111111e0_f64) * t71140 + F::cast_from(0.19931111111111111111e0_f64) * t71142;
    t71571
}
