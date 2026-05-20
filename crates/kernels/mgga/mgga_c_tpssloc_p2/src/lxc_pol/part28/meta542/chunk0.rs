//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1806/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1806<F: Float>(t22813: F, t6589: F, t80782: F, t23124: F, t23083: F, t23086: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t236: F, t2631: F) -> (F, F, F, F, F, F, F) {
    let t81902 = t22813 * t6589 * t80782;
    let t81903 = t81902 * t23124;
    let t81909 = t23083 * t23086;
    let t81911 = t23138 * t6604;
    let t81912 = t81911 * t6606;
    let t81914 = t22690 * t2627;
    let t81915 = t236 * t2631;
    (t81902, t81903, t81909, t81911, t81912, t81914, t81915)
}
