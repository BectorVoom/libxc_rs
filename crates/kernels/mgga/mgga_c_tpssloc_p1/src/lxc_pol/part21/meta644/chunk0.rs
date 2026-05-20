//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2435/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435<F: Float>(t10970: F, t820: F, t1041: F, t10868: F, t248: F, t2780: F, t10277: F, t976: F, t11046: F, t42387: F, t10457: F, t10936: F, t3180: F) -> (F, F, F, F, F, F) {
    let t42397 = t820 * t10970;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42444 = t976 * t10277;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42496 = t3180 * t10936;
    (t42397, t42432, t42444, t42483, t42488, t42496)
}
