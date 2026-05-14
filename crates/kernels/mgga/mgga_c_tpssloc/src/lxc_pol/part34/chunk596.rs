//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 596/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk596<F: Float>(t3: F, t6470: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t107: F, t625: F, t63: F, t656: F, t154: F, t781: F) -> (F, F, F, F, F) {
    let t6471 = t3 * t6470;
    let t6483 = 0.45e1 * t6470 * t577 + 27.0 * t5371 * t1458 + 27.0 * t3941 * t5456 + 0.135e2 * t1401 * t5493;
    let t6528 = t625 * t107;
    let t6530 = t63 * t656;
    let t6546 = t781 * t154;
    (t6471, t6483, t6528, t6530, t6546)
}
