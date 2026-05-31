//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2687/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687<F: Float>(t54432: F, t54434: F, t39596: F, t39601: F, t19644: F, t225: F, t20038: F, t5353: F, t12030: F, t12444: F, t1323: F, t1372: F, t1375: F, t1385: F, t1386: F, t1843: F, t19804: F, t20009: F, t20022: F, t20023: F, t20026: F, t20029: F, t3758: F, t3882: F, t3887: F, t3912: F, t53866: F, t54825: F, t55069: F, t55150: F, t568: F, t6440: F, t6461: F) -> (F, F, F, F, F) {
    let t56411 = F::cast_from(120.0_f64) * t54432;
    let t56412 = F::cast_from(0.10389515463408878255e3_f64) * t54434;
    let t56416 = F::cast_from(192.0_f64) * t39596;
    let t56417 = F::cast_from(8.0_f64) * t39601;
    let t56422 = t19644 * t225;
    let t56434 = t20038 * t225;
    let t56443 = t5353 * t5353;
    let t56457 = F::cast_from(4.0_f64) * t1375 * t1385 * t20022 * t3887 + F::cast_from(2.0_f64) * t1323 * t20009 * t568 + F::cast_from(2.0_f64) * t1372 * t19804 * t568 + F::cast_from(4.0_f64) * t1375 * t3887 * t56443 - t12030 * t6461 + F::cast_from(4.0_f64) * t12444 * t6440 - F::cast_from(2.0_f64) * t12444 * t6461 - F::cast_from(4.0_f64) * t1386 * t56422 - F::cast_from(2.0_f64) * t1386 * t56434 - F::cast_from(4.0_f64) * t1843 * t53866 - F::cast_from(2.0_f64) * t1843 * t54825 - F::cast_from(2.0_f64) * t1843 * t55069 - F::cast_from(2.0_f64) * t1843 * t55150 - F::cast_from(2.0_f64) * t20023 * t3758 + F::cast_from(4.0_f64) * t20026 * t3882 - F::cast_from(2.0_f64) * t20029 * t3912;
    (t56411, t56412, t56416, t56417, t56457)
}
