//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2151/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2151<F: Float>(t1824: F, t7722: F, t1338: F, t28107: F, t1336: F, t1352: F, t16047: F, t1814: F, t1825: F, t19654: F, t19744: F, t26401: F, t26403: F, t26453: F, t5250: F, t5287: F, t5334: F, t5344: F, t81147: F, t81149: F, t81154: F, t81187: F, t81197: F, t90952: F, t97158: F, t97161: F, t97172: F, t97179: F, t97181: F) -> (F, F) {
    let t97189 = t7722 * t1824;
    let t97193 = t1338 * t28107;
    let t97196 = F::cast_from(0.49348022005446793095e-1_f64) * t97158 - F::cast_from(0.24674011002723396548e-1_f64) * t97161 - t81147 - F::cast_from(2.0_f64) * t1336 * t90952 * t1825 + F::cast_from(2.0_f64) * t1814 * t26401 - F::cast_from(2.0_f64) * t5344 * t26403 * t5287 - F::cast_from(0.82246703342411321824e-2_f64) * t81149 + t81154 - F::cast_from(6.0_f64) * t16047 * t97172 * t19744 + F::cast_from(6.0_f64) * t5334 * t97172 * t5250 - F::cast_from(0.11514538467937585055e0_f64) * t97179 + F::cast_from(2.0_f64) * t5334 * t97181 * t5250 + F::cast_from(4.0_f64) * t19654 * t26453 - F::cast_from(0.12793931631041761173e0_f64) * t81187 + F::cast_from(0.16449340668482264365e-1_f64) * t81197 + F::cast_from(4.0_f64) * t5334 * t97189 * t5250 - t1336 * t97193 * t1352;
    (t97189, t97196)
}
