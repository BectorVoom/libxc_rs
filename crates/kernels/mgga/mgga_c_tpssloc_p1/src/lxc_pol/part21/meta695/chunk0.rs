//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2522/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2522<F: Float>(t10469: F, t1603: F, t11058: F, t11045: F, t11064: F, t10236: F, t14165: F, t13831: F, t13847: F, t2986: F, t10254: F, t12648: F) -> (F, F, F, F, F, F, F) {
    let t47840 = t1603 * t10469;
    let t47841 = t47840 * t11058;
    let t47853 = t47840 * t11045;
    let t47857 = t47840 * t11064;
    let t47887 = t10236 * t14165;
    let t47907 = t2986 * t13847 * t13831;
    let t47919 = t10254 * t12648;
    (t47840, t47841, t47853, t47857, t47887, t47907, t47919)
}
