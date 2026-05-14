//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1025/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1025<F: Float>(t10566: F, t10568: F, t10686: F, t10692: F, t14119: F, t14129: F, t14137: F, t14138: F, t14139: F, t14140: F, t14141: F, t14144: F, t14145: F, t8126: F, t8222: F, t10706: F, t10719: F, t14147: F, t14156: F, t14157: F, t14160: F, t14162: F, t14163: F, t14165: F, t14168: F, t7979: F, t7988: F, t7992: F, t8225: F, t8231: F, t8234: F) -> (F, F) {
    let t14264 = t14119 + t14129 - t8126 - t14137 - t14138 + t10566 + t10568 - t10686 + t14139 + t10692 - t14140 + t14141 + t14144 + t14145 + t8222;
    let t14265 = t8225 + t14147 - t8231 - t8234 + t7979 + t10706 + t14156 + t14157 + t14160 + t14162 + t14163 - t10719 + t14165 + t14168 + t7988 + t7992;
    (t14264, t14265)
}
