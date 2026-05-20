//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1925/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1925<F: Float>(t1510: F, t22986: F, t6646: F, t87111: F, t16820: F, t1888: F, t22996: F, t17031: F, t829: F, t98389: F, t16815: F, t9627: F) -> (F, F, F, F, F) {
    let t98461 = t22986 * t6646 * t87111 * t1510;
    let t98464 = t1888 * t22996 * t16820;
    let t98467 = t1888 * t22996 * t17031;
    let t98471 = t22986 * t6646 * t98389 * t829;
    let t98475 = t22986 * t22996 * t16815 * t9627;
    (t98461, t98464, t98467, t98471, t98475)
}
