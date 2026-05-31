//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1200/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1200<F: Float>(t12021: F, t12237: F, t1323: F, t1375: F, t2085: F, t24063: F, t24088: F, t24147: F, t3882: F, t3888: F, t568: F, t7213: F, t81333: F, t81339: F, t81346: F, t81350: F, t81365: F, t81375: F) -> F {
    let t84688 = F::cast_from(0.29608813203268075857e0_f64) * t81333 - F::cast_from(0.9869604401089358619e-1_f64) * t81339 + F::cast_from(0.9869604401089358619e-1_f64) * t81346 - F::cast_from(0.46058153871750340221e0_f64) * t81350 + F::cast_from(3.0_f64) * t1323 * t24063 * t568 + F::cast_from(6.0_f64) * t3882 * t24088 + F::cast_from(0.9869604401089358619e-1_f64) * t81365 + F::cast_from(12.0_f64) * t3882 * t24147 - F::cast_from(18.0_f64) * t1375 * t12021 * t7213 * t3888 - F::cast_from(0.76763589786250567036e0_f64) * t81375 + t12237 * t2085 * t568;
    t84688
}
