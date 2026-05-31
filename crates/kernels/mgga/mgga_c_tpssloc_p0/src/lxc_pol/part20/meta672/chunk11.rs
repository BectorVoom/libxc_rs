//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2537/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2537<F: Float>(t1147: F, t14933: F, t3400: F, t4832: F, t11282: F, t1687: F, t1129: F, t11311: F, t1137: F, t11400: F, t11410: F, t1157: F, t15118: F, t15121: F, t1695: F, t3327: F, t3396: F, t3404: F, t44183: F, t4820: F, t4835: F, t50821: F, t51119: F, t51122: F, t51124: F, t51126: F, t51128: F, t51267: F, t51279: F, t51293: F, t51306: F, t51320: F, t51332: F, t51346: F, t51359: F) -> F {
    let t51366 = t14933 * t1147;
    let t51371 = t4832 * t3400;
    let t51376 = t1687 * t11282;
    let t51381 = t50821 - t51119 - t51122 - t51124 - t51126 - t51128 + F::cast_from(3.0_f64) * t11410 * t4820 + F::cast_from(3.0_f64) * t3327 * t15118 + F::cast_from(1.0_f64) * t1129 * (t51267 + t51279 + t51293 + t51306 + t51320 + t51332 + t51346 + t51359) * t1137 + F::cast_from(0.17544670867903938621e1_f64) * t51366 * t1157 + F::cast_from(0.17544670867903938621e1_f64) * t15121 * t3396 + F::cast_from(0.51947577317044391276e2_f64) * t51371 * t3404 + F::cast_from(0.5848223622634646207e0_f64) * t4835 * t11400 + F::cast_from(0.10254018858216406658e4_f64) * t51376 * t11311 + F::cast_from(0.5848223622634646207e0_f64) * t44183 * t1695;
    t51381
}
