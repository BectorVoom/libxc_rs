//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1196/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1196<F: Float>(t12679: F, t7029: F, t18547: F, t1270: F, t3234: F, t5708: F, t1760: F, t1163: F, t1273: F, t1753: F, t1757: F, t18380: F, t18384: F, t18386: F, t18388: F, t18389: F, t18406: F, t18411: F, t18414: F, t18430: F, t18535: F, t18537: F, t18542: F, t18545: F, t2056: F, t2062: F, t3396: F, t485: F, t544: F, t5512: F, t5536: F, t5702: F, t626: F) -> (F, F, F, F) {
    let t18548 = t7029 * t12679;
    let t18550 = 6.0 * t18547 * t18548;
    let t18551 = t1270 * t3234;
    let t18552 = t5708 * t18551;
    let t18554 = 3.0 * t1760 * t18552;
    let t18561 = -2.0 * t1163 * t5512 + 2.0 * t1273 * t5702 - 2.0 * t1753 * t2062 + t1757 * t3396 - 4.0 * t18380 * t626 - 2.0 * t18389 * t626 - 2.0 * t18414 * t485 + t18430 * t544 - 4.0 * t2056 * t5536 - t18384 - t18386 - t18388 - t18406 - t18411 + t18535 + t18537 + t18542 + t18545 - t18550 + t18554;
    (t18548, t18551, t18552, t18561)
}
