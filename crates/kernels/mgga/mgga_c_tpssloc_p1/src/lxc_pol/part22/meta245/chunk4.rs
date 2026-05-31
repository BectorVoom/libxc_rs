//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1352/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1352<F: Float>(t1023: F, t10508: F, t248: F, t1020: F, t2928: F, t320: F) -> (F, F, F) {
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10523 = F::cast_from(1.0_f64) / t2928 / t320;
    (t10510, t10511, t10523)
}
