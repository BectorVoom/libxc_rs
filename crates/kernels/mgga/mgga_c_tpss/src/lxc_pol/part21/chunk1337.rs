//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1337/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1337<F: Float>(t18290: F, t6243: F, t18547: F, t41839: F, t7029: F, t1760: F, t18533: F, t4525: F, t18544: F, t6246: F, t1778: F, t41867: F, t116: F, t19430: F, t507: F, t6273: F) -> (F, F, F, F, F, F, F) {
    let t65480 = 6.0 * t6243 * t18290;
    let t65483 = 3.0 * t18547 * t7029 * t41839;
    let t65485 = t1760 * t18533 * t4525;
    let t65487 = 3.0 * t18544 * t6246;
    let t65489 = t1760 * t1778 * t41867;
    let t65490 = t19430 * t116;
    let t65497 = t507 * t6273;
    (t65480, t65483, t65485, t65487, t65489, t65490, t65497)
}
