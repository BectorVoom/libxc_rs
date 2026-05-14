//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1003/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1003<F: Float>(t12279: F, t12284: F, t12286: F, t12291: F, t12293: F, t12297: F, t12301: F, t12305: F, t12308: F, t12310: F, t12313: F, t12348: F, t12390: F, t12432: F, t1315: F, t1363: F, t3790: F, t3795: F, t5246: F) -> (F,) {
    let t12434 = t5246 * t12279 / 512.0 - 7.0 / 192.0 * t12284 + t12286 * t3795 / 512.0 - t12291 * t12293 / 512.0 + t3790 * t12297 / 512.0 + 7.0 / 768.0 * t12301 + 5.0 / 256.0 * t1363 * t12305 - 35.0 / 72.0 * t12308 + 7.0 / 48.0 * t12310 - t1315 * t12313 / 48.0 + t12348 + t12390 + t12432;
    (t12434,)
}
