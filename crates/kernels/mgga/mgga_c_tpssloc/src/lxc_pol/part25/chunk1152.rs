//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1152/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1152<F: Float>(t23124: F, t81902: F, t2628: F, t6605: F, t9981: F, t23083: F, t23086: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F) -> (F, F, F, F, F) {
    let t81903 = t81902 * t23124;
    let t81907 = t6605 * t2628 * t9981;
    let t81909 = t23083 * t23086;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    (t81903, t81907, t81909, t81912, t81914)
}
