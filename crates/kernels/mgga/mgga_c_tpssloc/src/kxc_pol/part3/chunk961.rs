//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 961/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk961<F: Float>(t13532: F, t4510: F, t10213: F, t60: F, t344: F, t13537: F, t10186: F, t10192: F, t10226: F, t10229: F, t13770: F, t13782: F, t13787: F, t13790: F, t13791: F, t2986: F, t4511: F, t4515: F, t4519: F) -> (F,) {
    let t13794 = t4510 * t13532;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13799 = t13798 * t13537;
    let t13804 = -0.18518518518518518518e-3 * t10192 - 0.37037037037037037036e-3 * t2986 * t13770 + 0.29629629629629629628e-2 * t10186 * t4519 - 0.19753086419753086419e-2 * t10186 * t4511 + 0.14814814814814814814e-2 * t10186 * t4515 - t13782 + t13787 - t13790 + 0.74074074074074074072e-3 * t2986 * t13791 + 0.37037037037037037036e-3 * t2986 * t13794 + 0.86419753086419753084e-3 * t2986 * t13799 - 0.12345679012345679012e-3 * t10226 + 0.9259259259259259259e-4 * t10229;
    (t13804,)
}
