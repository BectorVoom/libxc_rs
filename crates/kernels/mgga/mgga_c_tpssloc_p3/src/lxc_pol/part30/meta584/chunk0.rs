//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1963/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1963<F: Float>(t1351: F, t6387: F, t6330: F, t12250: F, t1834: F, t5286: F, t1824: F, t5318: F, t1372: F, t6414: F, t19731: F, t562: F) -> (F, F, F, F, F, F, F, F) {
    let t57091 = t6387 * t1351;
    let t57172 = t6330 * t1351;
    let t57342 = t6387 * t12250;
    let t57499 = t1834 * t5286;
    let t57545 = t5318 * t1824;
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57704 = t562 * t19731;
    (t57091, t57172, t57342, t57499, t57545, t57607, t57618, t57704)
}
