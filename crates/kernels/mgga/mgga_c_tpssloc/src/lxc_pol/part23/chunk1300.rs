//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1300/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1300<F: Float>(t1164: F, t43689: F, t43692: F, t78287: F, t18622: F, t64451: F, t21833: F, t4869: F, t5989: F, t64257: F, t11292: F, t1156: F, t22237: F, t78242: F, t78247: F, t78250: F, t78254: F, t78281: F, t78283: F, t78286: F) -> (F, F, F, F, F, F, F) {
    let t78291 = 0.91082604192152556044e5 * t1164 * t43689 * t78287 * t43692;
    let t78294 = 0.61524113149298439947e4 * t1164 * t64451 * t18622;
    let t78296 = 0.14035736694323150897e2 * t4869 * t21833;
    let t78298 = 12.0 * t64257 * t5989;
    let t78302 = 0.14035736694323150897e2 * t1164 * t11292 * t78287 * t1156;
    let t78304 = 0.4101607543286562663e4 * t4869 * t22237;
    let t78305 = t78242 - t78247 + t78250 + t78254 - t78281 - t78283 + t78286 - t78291 - t78294 + t78296 - t78298 + t78302 - t78304;
    (t78291, t78294, t78296, t78298, t78302, t78304, t78305)
}
