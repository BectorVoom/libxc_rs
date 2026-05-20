//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2054/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2054<F: Float>(t23083: F, t23086: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t236: F, t2631: F, t23109: F, t2632: F, t10024: F, t1899: F) -> (F, F, F, F, F, F, F) {
    let t81909 = t23083 * t23086;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81915 = t236 * t2631;
    let t81918 = t23109 * t81914 * t81915 * t2632;
    let t81920 = t1899 * t10024;
    (t81909, t81911, t81912, t81914, t81915, t81918, t81920)
}
