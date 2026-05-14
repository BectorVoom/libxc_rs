//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1020/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1020<F: Float>(t3791: F, t562: F, t10: F, t2229: F, t116: F, t117: F, t556: F, t252: F, t2631: F, t243: F, t828: F, t852: F, t3034: F, t371: F) -> (F, F, F, F, F, F, F, F) {
    let t22740 = t562 * t3791;
    let t22811 = t2229 * t10;
    let t22815 = t117 * t116;
    let t22842 = t556 * t556;
    let t22843 = 1.0 / t22842;
    let t22997 = t252 * t2631;
    let t23075 = t243 * t243;
    let t23076 = 1.0 / t23075;
    let t23175 = t852 * t828;
    let t23508 = 1.0 / t3034 / t371;
    (t22740, t22811, t22815, t22843, t22997, t23076, t23175, t23508)
}
