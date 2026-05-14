//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 746/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk746<F: Float>(t225: F, t4552: F, t68: F, t369: F, t1031: F, t1611: F, t1036: F, t1612: F, t1616: F, t248: F, t3101: F, t1020: F, t1044: F, t4347: F, t1009: F, t1603: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4615 = t4552 * t225;
    let t4616 = t4615 * t68;
    let t4617 = t4616 * t369;
    let t4622 = t1611 * t1031;
    let t4625 = t1612 * t1036;
    let t4630 = t248 * t3101 * t1616;
    let t4631 = t1020 * t4630;
    let t4636 = t248 * t1044 * t4347;
    let t4639 = t1603 * t1009;
    (t4615, t4616, t4617, t4622, t4625, t4630, t4631, t4636, t4639)
}
