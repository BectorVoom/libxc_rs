//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1425/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1425<F: Float>(t1406: F, t2239: F, t1409: F, t9321: F, t2291: F, t3966: F, t584: F, t9212: F, t9330: F, t2298: F, t2267: F, t2274: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12571 = t1406 * t2239;
    let t12595 = t9321 * t1409;
    let t12598 = t2291 * t3966;
    let t12603 = F::cast_from(2.0_f64) * t584;
    let t12604 = F::cast_from(6.0_f64) * t9212;
    let t12609 = t9330 * t1409;
    let t12612 = t2298 * t3966;
    let t12680 = t2267 * t3966;
    let t12698 = t2274 * t3966;
    (t12571, t12595, t12598, t12603, t12604, t12609, t12612, t12680, t12698)
}
