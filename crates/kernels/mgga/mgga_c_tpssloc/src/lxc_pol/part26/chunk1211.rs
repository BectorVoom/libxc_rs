//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1211/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1211<F: Float>(t2121: F, t3427: F, t7381: F, t24574: F, t24795: F, t24799: F, t3590: F, t477: F, t7365: F, t85660: F, t1170: F, t24829: F, t131: F, t467: F, t50: F, t82510: F) -> (F, F, F, F, F, F, F) {
    let t85941 = t2121 * t3427 * t7381;
    let t85943 = t24574 * t24795;
    let t85945 = t24574 * t24799;
    let t85947 = t477 * t3590;
    let t85952 = t85660 * t7365;
    let t85955 = t2121 * t1170 * t24829;
    let t85963 = t50 * t82510 * t131 * t467;
    (t85941, t85943, t85945, t85947, t85952, t85955, t85963)
}
