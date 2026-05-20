//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 645/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk645<F: Float>(t676: F, t724: F, t164: F, t723: F, t159: F, t730: F) -> (F, F, F, F, F) {
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = F::new(1.0) / t2458;
    let t2460 = t159 * t2459;
    let t2461 = t730 * t730;
    (t2454, t2458, t2459, t2460, t2461)
}
