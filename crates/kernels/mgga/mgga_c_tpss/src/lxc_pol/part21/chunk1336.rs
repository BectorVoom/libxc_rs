//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1336/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1336<F: Float>(t114: F, t65457: F, t485: F, t626: F, t18544: F, t6275: F, t19614: F, t5706: F, t10461: F, t1163: F, t12664: F, t13131: F, t13244: F, t1684: F, t1757: F, t17916: F, t19431: F, t20078: F, t2054: F, t2062: F, t3166: F, t3542: F, t5514: F, t6096: F, t6228: F, t624: F, t65134: F, t65138: F, t65141: F, t65143: F, t65429: F, t65436: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t65458 = piecewise3(t115, 0.0, t65457);
    let t65461 = 2.0 * t626 * t485 * t65458;
    let t65472 = t18544 * t6275;
    let t65474 = 6.0 * t5706 * t19614;
    let t65475 = -4.0 * t10461 * t5514 - 2.0 * t1163 * t19431 - t12664 * t1684 + t13131 * t1757 - 2.0 * t13244 * t5514 - 4.0 * t17916 * t3542 - 2.0 * t20078 * t624 - t2054 * t6228 - 2.0 * t2062 * t6228 - t3166 * t6096 - t485 * t65429 + t65134 + t65138 + t65141 - t65143 + t65436 - t65461 + t65472 + t65474;
    (t65458, t65475)
}
