//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 904/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk904<F: Float>(t2332: F, t581: F, t681: F, t1992: F, t725: F, t2204: F, t2319: F, t721: F, t735: F, t2214: F, t719: F, t2210: F, t2222: F, t2341: F, t2240: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t8100 = t2332 * t581;
    let t8101 = t681 * t8100;
    let t8106 = t725 * t1992;
    let t8107 = t681 * t8106;
    let t8110 = t2319 * t2204 * t721;
    let t8112 = 0.35089341735807877242e1 * t735 * t8110;
    let t8114 = t2214 * t719;
    let t8115 = t2210 * t2204 * t8114;
    let t8117 = 0.51947577317044391277e2 * t735 * t8115;
    let t8118 = t2341 * t2222;
    let t8120 = t2240 * t72;
    (t8101, t8107, t8110, t8112, t8115, t8117, t8118, t8120)
}
