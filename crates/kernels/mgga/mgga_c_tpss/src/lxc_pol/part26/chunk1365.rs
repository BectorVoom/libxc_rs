//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1365/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1365<F: Float>(t72790: F, t72798: F, t72819: F, t72827: F, t72840: F, t73117: F, t73130: F, t73601: F, t1281: F, t16067: F, t16076: F, t1904: F, t22209: F, t4556: F, t4559: F, t5477: F, t548: F, t6067: F, t6552: F, t71085: F, t71087: F, t71091: F, t71093: F, t71097: F, t71100: F, t71103: F, t71106: F, t71108: F, t71110: F, t71112: F) -> (F, F) {
    let t73604 = t72790 + t72798 + t72819 + t72827 + t72840 + t73117 + t73130 + t73601;
    let t73617 = t548 * t73604 * param_d + 3.0 * t1281 * t22209 + 12.0 * t16067 * t1904 + 3.0 * t16076 * t1904 + 12.0 * t4556 * t6552 + 6.0 * t4559 * t6552 + 3.0 * t5477 * t6067 + t71085 + t71087 + t71091 + t71093 + t71097 + t71100 + t71103 + t71106 + t71108 + t71110 + t71112;
    (t73604, t73617)
}
