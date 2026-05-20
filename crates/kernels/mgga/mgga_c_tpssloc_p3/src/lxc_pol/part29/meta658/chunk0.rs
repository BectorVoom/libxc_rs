//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2185/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2185<F: Float>(t12823: F, t7468: F, t26003: F, t4034: F, t26351: F, t6883: F, t1992: F, t26355: F, t80650: F, t22635: F, t26354: F, t3911: F) -> (F, F, F, F, F) {
    let t90454 = F::new(2.0) * t12823 * t7468;
    let t90456 = F::new(4.0) * t4034 * t26003;
    let t90459 = t6883 * t26351;
    let t90460 = F::cast_from(0.38381794893125283518e-1_f64) * t90459;
    let t90462 = t1992 * t80650 * t26355;
    let t90466 = t1992 * t22635 * t26354 * t3911;
    (t90454, t90456, t90460, t90462, t90466)
}
