//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1339/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1339<F: Float>(t1339: F, t17916: F, t19457: F, t19462: F, t2065: F, t2106: F, t3396: F, t3502: F, t3538: F, t6239: F, t646: F, t65480: F, t65483: F, t65485: F, t65487: F, t65489: F, t65490: F, t65500: F, t65504: F, t65506: F, t65508: F, t65510: F, t65512: F, t65515: F, t65518: F) -> (F,) {
    let t65523 = -2.0 * t1339 * t65518 - 4.0 * t17916 * t3538 - 4.0 * t19457 * t3502 - 4.0 * t19462 * t2065 - 2.0 * t19462 * t2106 + t3396 * t6239 - 4.0 * t646 * t65490 + t65480 - t65483 - t65485 + t65487 - t65489 + t65500 + t65504 - t65506 - t65508 - t65510 - t65512 + t65515;
    (t65523,)
}
