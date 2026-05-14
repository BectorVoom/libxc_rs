//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1253/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1253<F: Float>(t1003: F, t1022: F, t10359: F, t1058: F, t1060: F, t1063: F, t11007: F, t11027: F, t11031: F, t11043: F, t11065: F, t11066: F, t14590: F, t3180: F, t3186: F, t3188: F, t3189: F, t3196: F, t353: F, t383: F, t43419: F, t43480: F, t43483: F, t43489: F, t43503: F, t43504: F, t43505: F, t4673: F) -> (F,) {
    let t43512 = 4.0 * t1022 * t1058 * t1060 * t11007 + 8.0 * t11027 * t3186 * t4673 - 24.0 * t11065 * t11066 * t43483 - 36.0 * t11065 * t14590 * t3196 + 12.0 * t3186 * t3188 * t43489 + t353 * t383 * t43419 - t43503 * t43504 * t43505 + 4.0 * t1003 * t11043 + 4.0 * t10359 * t1063 + 12.0 * t11031 * t3180 + 12.0 * t3189 * t43480;
    (t43512,)
}
