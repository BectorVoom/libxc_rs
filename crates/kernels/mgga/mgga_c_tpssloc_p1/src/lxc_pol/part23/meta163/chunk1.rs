//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 763/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk763<F: Float>(t1378: F, t6460: F, t1375: F, t1843: F, t5215: F, t5321: F, t568: F, t6362: F, t6364: F, t6435: F, t6440: F) -> (F, F) {
    let t6461 = t1378 * t6460;
    let t6463 = F::cast_from(2.0_f64) * t1375 * t6440 - t1375 * t6461 - F::cast_from(2.0_f64) * t1843 * t5215 - F::cast_from(2.0_f64) * t1843 * t5321 + t568 * t6362 + F::cast_from(2.0_f64) * t568 * t6364 + t568 * t6435;
    (t6461, t6463)
}
