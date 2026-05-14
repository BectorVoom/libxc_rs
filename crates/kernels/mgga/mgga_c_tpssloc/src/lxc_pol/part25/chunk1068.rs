//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1068/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1068<F: Float>(t12021: F, t12237: F, t1323: F, t1375: F, t2085: F, t24063: F, t24088: F, t24147: F, t3882: F, t3888: F, t568: F, t7213: F, t81333: F, t81339: F, t81346: F, t81350: F, t81365: F, t81375: F) -> (F,) {
    let t84688 = 0.29608813203268075857e0 * t81333 - 0.9869604401089358619e-1 * t81339 + 0.9869604401089358619e-1 * t81346 - 0.46058153871750340221e0 * t81350 + 3.0 * t1323 * t24063 * t568 + 6.0 * t3882 * t24088 + 0.9869604401089358619e-1 * t81365 + 12.0 * t3882 * t24147 - 18.0 * t1375 * t12021 * t7213 * t3888 - 0.76763589786250567036e0 * t81375 + t12237 * t2085 * t568;
    (t84688,)
}
