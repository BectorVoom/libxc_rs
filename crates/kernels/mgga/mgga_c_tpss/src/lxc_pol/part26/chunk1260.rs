//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1260/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1260<F: Float>(t19142: F, t6021: F, t1880: F, t9615: F, t3048: F, t6016: F, t35167: F, t452: F, t6040: F, t9519: F, t1889: F, t35290: F, t31455: F, t5965: F, t31464: F, t19191: F, t7690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63371 = t6021 * t19142;
    let t63383 = t9615 * t1880;
    let t63396 = t3048 * t6016;
    let t63426 = t35167 * t452;
    let t63441 = t6040 * t9519;
    let t63448 = t1889 * t35290;
    let t63492 = t31455 * t5965;
    let t63495 = t31464 * t5965;
    let t63498 = t7690 * t19191;
    (t63371, t63383, t63396, t63426, t63441, t63448, t63492, t63495, t63498)
}
