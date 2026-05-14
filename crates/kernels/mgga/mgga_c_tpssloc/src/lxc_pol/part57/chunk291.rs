//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 291/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk291<F: Float>(t225: F, t252: F, t258: F, t214: F, t1880: F, t119: F, t210: F) -> (F, F, F, F) {
    let t1882 = t252 * t225 * t258;
    let t1883 = t214 * t1882;
    let t1884 = t1880 * t1883;
    let t1887 = t210 * t119;
    (t1882, t1883, t1884, t1887)
}
