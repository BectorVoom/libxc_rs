//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1262/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1262<F: Float>(t1673: F, t6441: F, t1276: F, t1278: F, t1284: F, t16041: F, t16079: F, t1666: F, t1849: F, t1856: F, t20697: F, t21948: F, t21984: F, t4562: F, t5466: F, t5480: F, t5942: F, t5960: F, t6442: F, t67849: F, t67851: F, t67853: F, t71181: F, t72733: F) -> (F,) {
    let t72737 = t6441 * t1673;
    let t72743 = 2.0 * t6442 * t4562 + t21948 * t1284 + 2.0 * t1666 * t20697 + t1278 * (t71181 + t72733) + t67849 + t16041 * t1856 + t67851 + 2.0 * t72737 + t67853 + t5942 * t5480 + t5466 * t5960 + t1276 * t21984 + t1849 * t16079;
    (t72743,)
}
