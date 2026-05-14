//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1097/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1097<F: Float>(t11713: F, t1210: F, t53081: F, t11647: F, t1731: F, t11718: F, t52835: F, t1744: F, t11716: F, t1174: F, t1725: F, t2402: F, t11727: F, t11832: F, t1706: F, t11887: F, t52834: F) -> (F, F, F, F, F, F, F, F, F) {
    let t53087 = t11713 * t1210 * t53081;
    let t53099 = t1731 * t11647;
    let t53238 = t52835 * t11718;
    let t53274 = t1744 * t11647;
    let t53336 = t11713 * t11716 * t53081;
    let t53440 = t1174 * t2402 * t1725;
    let t53472 = t52835 * t11727;
    let t53490 = t1706 * t11832;
    let t53565 = t52834 * t11887;
    (t53087, t53099, t53238, t53274, t53336, t53440, t53472, t53490, t53565)
}
