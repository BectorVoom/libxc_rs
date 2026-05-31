//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1006/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1006<F: Float>(t28: F, t265: F, t504: F, t128193: F, t128239: F, t128278: F, t1409: F, t33547: F, t52: F, t5398: F, t8591: F, t113: F, t128201: F, t122617: F, t126127: F, t126132: F, t127720: F, t127722: F, t127726: F, t127728: F, t127730: F, t127736: F, t127738: F, t127742: F, t1459: F, t19451: F, t1976: F, t2040: F, t24999: F, t28943: F, t28959: F, t29205: F, t33085: F, t6517: F, t7796: F, t8529: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t128280 = piecewise3::<F>(t505, F::cast_from(0.0_f64), t128193);
    let t128287 = piecewise3::<F>(t401, t128239 + t128278, t128280 * t52 / F::cast_from(2.0_f64) - t33547 * t1409 - t8591 * t5398 / F::cast_from(2.0_f64));
    let t128289 = t113 * (t128201 + t128287);
    let t128293 = -F::cast_from(4.0_f64) * t122617 * t1459 - F::cast_from(4.0_f64) * t126127 * t2040 - F::cast_from(2.0_f64) * t126132 * t2040 - F::cast_from(2.0_f64) * t19451 * t8529 - t1976 * t28943 - F::cast_from(2.0_f64) * t1976 * t28959 - F::cast_from(4.0_f64) * t24999 * t7796 - F::cast_from(4.0_f64) * t29205 * t6517 - F::cast_from(4.0_f64) * t33085 * t7796 - t127720 - t127722 - t127726 - t127728 - t127730 + t127736 - t127738 - t127742 - t128289;
    t128293
}
