//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2337/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337<F: Float>(t12524: F, t28896: F, t3941: F, t5493: F, t6534: F, t100902: F, t100908: F, t100911: F, t100915: F, t100917: F, t100921: F, t100924: F, t100927: F, t100929: F, t100932: F, t100934: F, t100936: F, t1458: F, t19534: F, t20181: F, t23880: F, t5376: F, t671: F, t7010: F, t86647: F, t86656: F) -> F {
    let t100938 = F::new(54.0) * t12524 * t28896;
    let t100941 = F::new(27.0) * t3941 * t6534 * t5493;
    let t100942 = t100902 + F::new(54.0) * t86647 * t5376 + F::new(0.135e2) * t7010 * t19534 + t100908 + F::new(27.0) * t23880 * t20181 + F::new(0.135e2) * t100911 * t671 + t100915 + t100917 + F::new(27.0) * t86656 * t1458 + t100921 + t100924 + t100927 + t100929 + t100932 + t100934 + t100936 + t100938 + t100941;
    t100942
}
