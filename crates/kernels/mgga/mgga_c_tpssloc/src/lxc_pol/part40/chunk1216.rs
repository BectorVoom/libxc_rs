//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1216/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1216<F: Float>(t5427: F, t608: F, t5392: F, t9287: F, t607: F, t3966: F, t3981: F, t2267: F, t5398: F, t16558: F, t43: F, t9300: F) -> (F, F, F, F, F, F) {
    let t19363 = t608 * t5427;
    let t19368 = t9287 * t5392;
    let t19369 = t19368 * t607;
    let t19372 = t3981 * t3966;
    let t19377 = t2267 * t5398;
    let t19378 = t19377 * t607;
    let t19381 = t43 * t16558;
    let t19390 = t9300 * t5392;
    (t19363, t19369, t19372, t19378, t19381, t19390)
}
