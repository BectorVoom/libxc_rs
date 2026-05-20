//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1888/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1888<F: Float>(t1409: F, t2132: F, t2136: F, t460: F, t4928: F, t7320: F, t210: F, t7998: F) -> (F, F, F, F) {
    let t27650 = t2132 * t1409;
    let t27651 = t27650 * t2136;
    let t27654 = t4928 * t460;
    let t27655 = t27654 * t7320;
    let t27674 = t7998 * t210;
    (t27651, t27654, t27655, t27674)
}
