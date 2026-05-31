//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1280/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1280<F: Float>(t109: F, t5464: F, t8129: F, t1444: F, t1453: F, t8138: F, t5488: F, t29926: F, t5468: F, t5396: F, t29894: F, t29903: F, t30147: F, t30162: F, t8128: F, t8137: F) -> (F, F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t30407 = t8129 * t5464;
    let t30410 = t1453 * t1444;
    let t30411 = t8138 * t30410;
    let t30414 = t8129 * t5488;
    let t30417 = t29926 * t5468;
    let t30420 = t8138 * t5396;
    let t30424 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t29894 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t30147 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t30162 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t29903 * t30407 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8128 * t30411 + t8128 * t30414 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8137 * t30417 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8137 * t30420);
    (t30407, t30410, t30411, t30414, t30417, t30420, t30424)
}
