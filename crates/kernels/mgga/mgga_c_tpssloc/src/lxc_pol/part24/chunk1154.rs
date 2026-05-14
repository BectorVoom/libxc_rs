//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1154/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1154<F: Float>(t22776: F, t22779: F, t1307: F, t1339: F, t22827: F, t3856: F, t12251: F, t12289: F, t6936: F, t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F, t6604: F) -> (F, F, F, F, F, F) {
    let t80943 = t22779 * t22776;
    let t80947 = t22827 * t1339 * t3856 * t1307;
    let t80950 = t6936 * t12289 * t12251;
    let t80953 = 1.0 / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80957 = 0.69792532988666768264e-2 * t80956;
    let t80958 = t22803 * t6604;
    (t80943, t80947, t80950, t80953, t80957, t80958)
}
