//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1298/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1298<F: Float>(t121179: F, t121181: F, t121184: F, t121190: F, t121192: F, t121194: F, t122920: F, t124863: F, t2040: F, t2075: F, t27226: F, t27371: F, t27888: F, t33690: F, t510: F, t7050: F, t7266: F, t7802: F, t8329: F) -> (F,) {
    let t124947 = -2.0 * t122920 * t2040 - t124863 * t510 - t2075 * t27371 - 2.0 * t27226 * t7266 - 2.0 * t27888 * t7802 - 2.0 * t33690 * t7050 - t121179 - t121181 + t121184 - t121190 - t121192 - t121194 - t8329;
    (t124947,)
}
