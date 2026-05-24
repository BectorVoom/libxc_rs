//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 960/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk960<F: Float>(t3272: F, t774: F, t1232: F, t1639: F, t3260: F, t3342: F, t4480: F, t10077: F, t1642: F, t10160: F, t1630: F, t125: F, t4459: F) -> (F, F, F, F, F, F, F) {
    let t12822 = t3272 * t774;
    let t12823 = t1639 * t1232;
    let t12828 = t1639 * t3260;
    let t12835 = F::new(35.0) / F::new(576.0) * t3342 * t4480;
    let t12846 = t10077 * t1642;
    let t12861 = t10160 * t1630;
    let t12863 = t125 * t4459;
    (t12822, t12823, t12828, t12835, t12846, t12861, t12863)
}
