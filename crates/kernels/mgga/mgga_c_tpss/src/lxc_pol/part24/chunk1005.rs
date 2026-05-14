//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1005/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1005<F: Float>(t10566: F, t10568: F, t10686: F, t10692: F, t14119: F, t14123: F, t14129: F, t14130: F, t14137: F, t14138: F, t14139: F, t14140: F, t14141: F, t14144: F, t14145: F, t1692: F, t2439: F, t2440: F, t3548: F, t3552: F, t3683: F, t4701: F, t750: F, t8117: F, t8126: F, t821: F) -> (F,) {
    let t14146 = 2.0 * t14123 * t1692 * t821 - 3.0 * t14130 * t2439 * t750 + 3.0 * t2439 * t2440 * t4701 + 12.0 * t3548 * t3552 * t3683 + t10566 + t10568 - t10686 + t10692 + t14119 + t14129 - t14137 - t14138 + t14139 - t14140 + t14141 + t14144 + t14145 - t8117 - t8126;
    (t14146,)
}
