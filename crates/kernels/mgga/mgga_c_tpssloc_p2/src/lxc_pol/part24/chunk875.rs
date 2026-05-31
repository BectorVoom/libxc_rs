//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 875/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk875<F: Float>(t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4225: F, t822: F, t825: F, t9938: F, t9947: F, t9951: F, t9954: F) -> F {
    let t9957 = F::cast_from(60.0_f64) * t228 * t9947 + F::cast_from(3.0_f64) * t228 * t9954 - t230 * t9938 + F::cast_from(9.0_f64) * t2667 * t825 - F::cast_from(36.0_f64) * t2672 * t822 + F::cast_from(9.0_f64) * t2675 * t822 - F::cast_from(36.0_f64) * t4225 * t9951;
    t9957
}
