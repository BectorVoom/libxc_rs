//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 682/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk682<F: Float>(t1550: F, t68741: F, t14089: F, t34683: F, t7557: F, t14078: F, t7494: F, t1330: F, t22: F, t262: F, t2134: F, t219: F, t3147: F) -> (F, F, F, F, F, F) {
    let t68742 = t1550 * t68741;
    let t68751 = t14089 * t34683 * t7557;
    let t68753 = t7494 * t14078;
    let t68756 = t1330 * t22 * t262;
    let t68757 = t2134 * t68756;
    let t68759 = t3147 * t219;
    (t68742, t68751, t68753, t68756, t68757, t68759)
}
