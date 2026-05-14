//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 979/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk979<F: Float>(t2684: F, t4295: F, t13171: F, t860: F, t4265: F, t814: F, t829: F, t13377: F, t235: F, t2679: F, t4282: F, t4280: F, t808: F, t13384: F, t13176: F, t13336: F, t1499: F, t1523: F, t1525: F, t226: F, t255: F, t2613: F, t2617: F, t2738: F, t2740: F, t4162: F, t4166: F, t4283: F, t4286: F, t4288: F, t4291: F, t4298: F, t812: F, t861: F, t863: F, t9612: F) -> (F,) {
    let t13429 = t4295 * t2684;
    let t13431 = t860 * t13171;
    let t13433 = t814 * t4265;
    let t13434 = t13433 * t829;
    let t13448 = t235 * t13377;
    let t13450 = t4282 * t2679;
    let t13453 = t808 * t4280;
    let t13456 = t13384 * t829;
    let t13459 = -2.0 * t13176 * t861 + t13336 * t255 - t13429 * t812 - t13431 * t812 - 2.0 * t13434 * t812 + t13448 * t226 - t13450 * t4291 + 4.0 * t13453 * t4283 - 2.0 * t13456 * t4291 + t1499 * t2740 - t1523 * t9612 + t1525 * t2613 - 2.0 * t2617 * t4286 - 2.0 * t2617 * t4288 - t2738 * t4166 + 2.0 * t4162 * t863 + 2.0 * t4298 * t808;
    (t13459,)
}
