//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2716/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2716<F: Float>(t1266: F, t1393: F, t1774: F, t19450: F, t19451: F, t19461: F, t19534: F, t20293: F, t20347: F, t20350: F, t20720: F, t2314: F, t4034: F, t4073: F, t510: F, t5107: F, t5118: F, t5450: F, t5457: F, t6468: F, t652: F, t75555: F) -> F {
    let t75733 = -F::new(2.0) * t1266 * t20347 * t652 - F::new(6.0) * t1774 * t19534 * t652 - t1266 * t20293 + t1393 * t20350 - F::new(3.0) * t1774 * t19450 - F::new(6.0) * t1774 * t19461 - F::new(6.0) * t19451 * t4073 - F::new(2.0) * t20720 * t2314 - F::new(2.0) * t20720 * t4034 - t510 * t75555 - F::new(3.0) * t5107 * t5450 - F::new(6.0) * t5107 * t5457 + F::new(3.0) * t5118 * t6468;
    t75733
}
