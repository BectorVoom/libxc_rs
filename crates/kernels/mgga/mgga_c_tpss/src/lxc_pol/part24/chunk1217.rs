//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1217/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1217<F: Float>(t21190: F, t485: F, t626: F, t1600: F, t1684: F, t1753: F, t21014: F, t21016: F, t21020: F, t21021: F, t21026: F, t21030: F, t21035: F, t21109: F, t21111: F, t21114: F, t21171: F, t21177: F, t21179: F, t21182: F, t21184: F, t4638: F, t5314: F, t6096: F) -> (F, F) {
    let t21191 = t485 * t21190;
    let t21193 = 2.0 * t626 * t21191;
    let t21194 = -2.0 * t1600 * t6096 - t1684 * t5314 - 2.0 * t1753 * t4638 - 2.0 * t21021 * t485 - t21171 * t485 - t21014 - t21016 + t21020 + t21026 + t21030 + t21035 + t21109 - t21111 + t21114 - t21177 - t21179 - t21182 - t21184 - t21193;
    (t21191, t21194)
}
