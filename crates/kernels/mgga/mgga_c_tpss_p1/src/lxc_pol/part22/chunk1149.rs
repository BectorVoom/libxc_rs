//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1149/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1149<F: Float>(t10120: F, t774: F, t1232: F, t1625: F, t3275: F, t3272: F, t1639: F, t3260: F, t1206: F, t3342: F, t4480: F, t4397: F) -> (F, F, F, F, F, F, F) {
    let t12816 = t10120 * t774;
    let t12817 = t1625 * t1232;
    let t12818 = t12817 * t3275;
    let t12819 = t12816 * t12818;
    let t12822 = t3272 * t774;
    let t12823 = t1639 * t1232;
    let t12825 = t12822 * t12823 * t3275;
    let t12828 = t1639 * t3260;
    let t12829 = t1232 * t1206;
    let t12831 = t12822 * t12828 * t12829;
    let t12835 = F::new(35.0) / F::new(576.0) * t3342 * t4480;
    let t12836 = t4397 * t1206;
    (t12819, t12823, t12825, t12828, t12831, t12835, t12836)
}
