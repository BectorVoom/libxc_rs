//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1180/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1180<F: Float>(t18419: F, t18421: F, t18423: F, t18425: F, t18427: F, t18429: F, t19240: F, t19244: F, t19261: F, t2105: F, t5986: F, t645: F, t1273: F, t18388: F, t18406: F, t18411: F, t18535: F, t18537: F, t18542: F, t18545: F, t18550: F, t18554: F, t1897: F, t1899: F, t19251: F, t2054: F, t2056: F, t2062: F, t2065: F, t2106: F, t3396: F, t544: F, t5991: F, t6054: F, t6058: F, t624: F, t626: F, t646: F) -> (F, F) {
    let t19272 = 4.0 * t19261 * t645 + 2.0 * t2105 * t5986 + t18419 + t18421 + t18423 + t18425 + t18427 + t18429 + t19240 + 2.0 * t19244;
    let t19278 = 2.0 * t1273 * t6058 - t1897 * t2054 - 2.0 * t1897 * t2062 + t1899 * t3396 - 2.0 * t19251 * t626 - 4.0 * t19261 * t646 + t19272 * t544 - 4.0 * t2056 * t5991 - 4.0 * t2065 * t5986 - 2.0 * t2106 * t5986 - 2.0 * t6054 * t624 - t18388 - t18406 - t18411 + t18535 + t18537 + t18542 + t18545 - t18550 + t18554;
    (t19272, t19278)
}
