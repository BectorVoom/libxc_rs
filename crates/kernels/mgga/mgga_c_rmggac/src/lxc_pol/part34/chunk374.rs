//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 374/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk374<F: Float>(t290: F, t7352: F, t7755: F, t1343: F, t649: F, t640: F, t2064: F, t333: F, t265: F, t338: F) -> (F, F, F, F, F, F) {
    let t7756 = t290 * t7352;
    let t7757 = t7755 * t7756;
    let t7764 = t649 * t1343;
    let t7765 = t640 * t7352;
    let t7769 = t2064 * t333;
    let t7778 = t338 * t265;
    (t7756, t7757, t7764, t7765, t7769, t7778)
}
