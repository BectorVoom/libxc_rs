//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1330/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1330<F: Float>(t11719: F, t11728: F, t11738: F, t1227: F, t15438: F, t15659: F, t15737: F, t1735: F, t1743: F, t19056: F, t22115: F, t22271: F, t22275: F, t22314: F, t248: F, t3506: F, t3515: F, t3585: F, t4582: F, t488: F, t53472: F, t6225: F, t6230: F, t65474: F, t66015: F, t72669: F, t72673: F, t73028: F, t77965: F) -> (F,) {
    let t79160 = -7.0 / 486.0 * t72669 - t22115 * t1743 * t488 / 144.0 - t72673 / 72.0 + t15737 * t22271 / 128.0 + t3506 * t4582 * t73028 * t15659 / 384.0 + 3.0 / 256.0 * t11719 * t4582 * t19056 * t65474 - 3.0 / 256.0 * t11728 * t4582 * t19056 * t6225 - t15438 * t22275 / 256.0 - t3515 * t4582 * t73028 * t1735 / 768.0 + t11738 * t4582 * t19056 * t6230 / 512.0 + t66015 / 108.0 + 5.0 / 4608.0 * t1227 * t248 * t3585 * t77965 - t53472 * t22314 / 128.0;
    (t79160,)
}
