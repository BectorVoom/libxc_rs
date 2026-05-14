//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1160/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1160<F: Float>(t3: F, t30465: F, t1458: F, t8230: F, t2180: F, t5493: F, t1401: F, t16524: F, t20162: F, t28893: F, t29996: F, t30231: F, t30424: F, t3941: F, t5371: F, t5456: F, t577: F, t8161: F, t8251: F) -> (F, F, F, F) {
    let t30466 = t3 * t30465;
    let t30492 = t8230 * t1458;
    let t30495 = t2180 * t5493;
    let t30500 = 0.45e1 * t30465 * t577 + 27.0 * t30231 * t1458 + 27.0 * t29996 * t5456 + 0.135e2 * t8161 * t5493 + 0.135e2 * t20162 * t2180 + 54.0 * t16524 * t8251 + 27.0 * t5371 * t8230 + 27.0 * t28893 * t2180 + 54.0 * t3941 * t30492 + 27.0 * t3941 * t30495 + 0.135e2 * t1401 * t30424;
    (t30466, t30492, t30495, t30500)
}
