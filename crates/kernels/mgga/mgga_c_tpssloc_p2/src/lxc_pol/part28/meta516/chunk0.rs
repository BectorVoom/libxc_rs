//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1764/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1764<F: Float>(t13487: F, t57911: F, t1484: F, t2749: F, t4303: F, t868: F, t4119: F, t4233: F, t829: F, t16935: F, t828: F, t2745: F) -> (F, F, F, F, F, F, F) {
    let t57912 = t57911 * t13487;
    let t57921 = t1484 * t2749;
    let t58009 = t4303 * t868;
    let t58071 = t4119 * t868;
    let t58300 = t829 * t4233;
    let t58345 = t16935 * t828;
    let t59580 = t1484 * t2745;
    (t57912, t57921, t58009, t58071, t58300, t58345, t59580)
}
