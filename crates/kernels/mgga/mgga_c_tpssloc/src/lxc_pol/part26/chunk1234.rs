//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1234/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1234<F: Float>(t85442: F, t85585: F, t85595: F, t86548: F, t12529: F, t12532: F, t2169: F, t2319: F, t2363: F, t24969: F, t24972: F, t577: F, t671: F, t7423: F, t83979: F, t83984: F, t83988: F, t83991: F, t83993: F, t83999: F, t84001: F, t84003: F, t84009: F, t84012: F, t84014: F, t84016: F, t84018: F, t85416: F, t85423: F, t9416: F) -> (F, F) {
    let t86550 = t85442 + t85585 + t85595 + t86548;
    let t86553 = 0.135e2 * t7423 * t9416 + 81.0 * t85416 * t2319 + 0.405e2 * t24969 * t2363 + t83979 + t83984 + 27.0 * t2169 * t12529 + 0.405e2 * t85423 * t671 + 81.0 * t24972 * t12532 + t83988 + 0.45e1 * t86550 * t577 + t83991 + t83993 + t83999 + t84001 + t84003 + t84009 + t84012 + t84014 + t84016 + t84018;
    (t86550, t86553)
}
