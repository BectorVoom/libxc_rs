//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 892/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk892<F: Float>(t225: F, t24064: F, t2056: F, t40772: F, t24334: F, t2752: F, t193: F, t201: F, t7109: F, t10143: F, t2047: F, t2678: F) -> (F, F, F, F, F, F) {
    let t84700 = t24064 * t225;
    let t84766 = t2056 * t40772;
    let t84791 = t24334 * t2752;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t84842 = t2047 * t2678;
    (t84700, t84766, t84791, t84797, t84800, t84842)
}
