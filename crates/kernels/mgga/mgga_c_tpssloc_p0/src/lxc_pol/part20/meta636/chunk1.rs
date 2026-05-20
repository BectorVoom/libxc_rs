//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2338/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338<F: Float>(t10236: F, t14165: F, t13831: F, t13847: F, t2986: F, t10913: F, t4337: F, t10254: F, t12648: F, t43070: F, t10190: F, t13835: F) -> (F, F, F, F, F, F) {
    let t47887 = t10236 * t14165;
    let t47907 = t2986 * t13847 * t13831;
    let t47915 = t4337 * t10913;
    let t47919 = t10254 * t12648;
    let t47927 = t43070 * t14165;
    let t47938 = t2986 * t10190 * t13835;
    (t47887, t47907, t47915, t47919, t47927, t47938)
}
