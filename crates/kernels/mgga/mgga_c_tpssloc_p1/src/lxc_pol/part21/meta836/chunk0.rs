//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2970/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970<F: Float>(t17997: F, t3070: F, t42488: F, t1041: F, t13969: F, t17975: F, t10413: F, t10876: F, t10937: F, t14080: F, t1409: F, t14167: F, t14172: F, t14218: F, t14219: F, t17649: F, t17712: F, t17920: F, t17923: F, t3071: F, t3131: F, t3132: F, t3966: F, t42483: F, t43361: F, t4579: F, t4582: F, t4590: F, t4644: F, t49604: F, t49607: F, t49621: F, t49629: F, t49984: F, t61910: F, t883: F) -> F {
    let t61916 = t3070 * t42488 * t17997;
    let t61919 = t1041 * t13969 * t17975;
    let t61921 = t10937 * t17649 / F::cast_from(216.0_f64) + t49604 / F::cast_from(1728.0_f64) + t49607 / F::cast_from(1728.0_f64) - t49984 * t4579 / F::cast_from(216.0_f64) - t10876 * t4582 * t17712 * t3132 / F::cast_from(512.0_f64) - t10413 * t3071 * t14218 * t14219 * t3966 / F::cast_from(1152.0_f64) - t43361 * t3071 * t49621 * t3131 * t883 * t1409 / F::cast_from(384.0_f64) + t42483 * t3071 * t49621 * t17923 / F::cast_from(2304.0_f64) - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t10937 * t17920 + t49629 / F::cast_from(864.0_f64) + t4644 * t14167 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t14080 * t4590 - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1041 * t4582 * t14172 * t61910 + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t61916 - t61919 / F::cast_from(864.0_f64);
    t61921
}
