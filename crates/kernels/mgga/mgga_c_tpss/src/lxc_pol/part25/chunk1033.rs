//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1033/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1033<F: Float>(t10566: F, t10568: F, t10686: F, t10692: F, t14119: F, t14129: F, t14137: F, t14138: F, t14139: F, t14140: F, t14141: F, t14144: F, t14145: F, t8126: F, t8222: F) -> F {
    let t14264 = t14119 + t14129 - t8126 - t14137 - t14138 + t10566 + t10568 - t10686 + t14139 + t10692 - t14140 + t14141 + t14144 + t14145 + t8222;
    t14264
}
