//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1256/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1256<F: Float>(t1107: F, t19103: F, t3048: F, t6016: F, t19112: F, t6030: F, t35167: F, t452: F, t19164: F, t3154: F, t6040: F, t9519: F, t1889: F, t35290: F, t31455: F, t5965: F) -> (F, F, F, F, F, F, F, F) {
    let t63392 = t1107 * t19103;
    let t63396 = t3048 * t6016;
    let t63419 = t19112 * t6030;
    let t63426 = t35167 * t452;
    let t63437 = t19164 * t3154;
    let t63441 = t6040 * t9519;
    let t63448 = t1889 * t35290;
    let t63492 = t31455 * t5965;
    (t63392, t63396, t63419, t63426, t63437, t63441, t63448, t63492)
}
