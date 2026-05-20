//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2338/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338<F: Float>(t28904: F, t576: F, t28868: F, t580: F, t100900: F, t100942: F, t1398: F, t1404: F, t1858: F, t20149: F, t20186: F, t2023: F, t2029: F, t26510: F, t28869: F, t5364: F, t6471: F, t7020: F, t7774: F, t86565: F, t86567: F, t86571: F, t96348: F) -> F {
    let t100945 = t576 * t28904;
    let t100946 = t28868 * t580;
    let t100948 = t86565 + F::new(2.0) * t26510 * t1858 + t86567 + F::new(2.0) * t5364 * t7774 + t6471 * t7020 + t28869 * t1404 + t20149 * t2029 + t96348 + t1398 * (t100900 + t100942) + t100945 + t86571 + t100946 + t2023 * t20186;
    t100948
}
