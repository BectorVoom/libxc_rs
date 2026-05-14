//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1045/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1045<F: Float>(t30: F, t33: F, t2: F, t3282: F, t555: F, t580: F, t12696: F, t1991: F, t22: F, t3218: F, t4360: F, t4363: F, t490: F, t1497: F, t9868: F, t3289: F, t1006: F, t2829: F, t3226: F, t4368: F, t4371: F, t493: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t12699 = t3282 * t2;
    let t12700 = t555 * t580;
    let t12710 = piecewise3(t31, 0.0, -8.0 / 27.0 * t12696 * t3218 + 16.0 / 9.0 * t12699 * t12700 + 4.0 / 9.0 * t4360 * t1991 + 8.0 / 3.0 * t490 * t555 - 8.0 * t4363 * t22);
    let t12711 = t9868 * t1497;
    let t12714 = t3289 * t2;
    let t12715 = t555 * t1006;
    let t12725 = piecewise3(t34, 0.0, -8.0 / 27.0 * t12711 * t3226 - 16.0 / 9.0 * t12714 * t12715 + 4.0 / 9.0 * t4368 * t2829 - 8.0 / 3.0 * t493 * t555 + 8.0 * t4371 * t22);
    (t12700, t12710, t12715, t12725)
}
