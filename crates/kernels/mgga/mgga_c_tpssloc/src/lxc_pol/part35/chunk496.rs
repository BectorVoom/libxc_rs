//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 496/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk496<F: Float>(t1089: F, t460: F, t3247: F, t461: F, t3293: F, t3030: F, t466: F, t3032: F) -> (F, F, F, F, F) {
    let t3450 = t460 * t1089;
    let t3455 = t461 * t3247;
    let t3464 = 5.0 / 18.0 * t3293;
    let t3499 = t466 * t3030;
    let t3500 = t3499 * t3032;
    (t3450, t3455, t3464, t3499, t3500)
}
