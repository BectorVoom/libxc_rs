//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2481/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481<F: Float>(t18030: F, t4630: F, t17884: F, t4644: F, t13969: F, t21502: F, t3039: F, t10214: F, t1041: F, t14080: F, t14164: F, t21603: F, t2979: F, t3048: F, t4582: F, t47775: F, t5861: F, t62282: F, t62284: F, t68521: F, t68534: F, t68539: F, t70330: F, t70339: F, t973: F, t977: F) -> (F, F) {
    let t70554 = t18030 * t4630;
    let t70573 = t4644 * t17884;
    let t70597 = t3039 * t13969 * t21502;
    let t70599 = -t3048 * t21603 / F::new(864.0) + F::new(5.0) / F::new(6912.0) * t70573 - t62282 / F::new(216.0) - t62284 / F::new(3456.0) - F::new(5.0) / F::new(864.0) * t14080 * t5861 - F::new(7.0) / F::new(54.0) * t973 * t10214 * t68521 - t973 * t977 * t68534 / F::new(144.0) + t973 * t2979 * t68539 / F::new(216.0) - t1041 * t4582 * t47775 * t70330 / F::new(192.0) + t1041 * t4582 * t14164 * t70339 / F::new(256.0) - t70597 / F::new(1536.0);
    (t70554, t70599)
}
