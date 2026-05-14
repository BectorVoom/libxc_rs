//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1166/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1166<F: Float>(t3: F, t30094: F, t112: F, t8199: F, t111: F, t2205: F, t671: F, t8189: F, t2199: F, t2363: F, t12521: F, t12524: F, t1401: F, t16535: F, t2319: F, t30071: F, t3938: F, t3941: F, t577: F, t8207: F, t8212: F) -> (F, F, F, F, F, F) {
    let t30095 = t3 * t30094;
    let t30109 = t8199 * t112;
    let t30112 = t2205 * t111;
    let t30125 = t8189 * t671;
    let t30128 = t2199 * t2363;
    let t30133 = 0.45e1 * t30094 * t577 + 27.0 * t30109 * t671 + 27.0 * t30112 * t2319 + 0.135e2 * t8207 * t2363 + 0.135e2 * t12521 * t2199 + 54.0 * t12524 * t8212 + 27.0 * t3938 * t8189 + 27.0 * t16535 * t2199 + 54.0 * t3941 * t30125 + 27.0 * t3941 * t30128 + 0.135e2 * t1401 * t30071;
    (t30095, t30109, t30112, t30125, t30128, t30133)
}
