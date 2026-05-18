//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1382/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1382<F: Float>(t1015: F, t23520: F, t82895: F, t23563: F, t25650: F, t3082: F, t6750: F, t607: F, t984: F, t23562: F, t343: F, t3008: F, t40: F) -> (F, F, F, F, F, F) {
    let t82897 = t82895 * t1015 * t23520;
    let t82911 = t25650 * t23563;
    let t82914 = t6750 * t3082;
    let t82916 = t607 * t984;
    let t82918 = t23562 * t82916 * t343;
    let t82921 = t40 * t3008;
    (t82897, t82911, t82914, t82916, t82918, t82921)
}
