//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1296/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1296<F: Float>(t120995: F, t120998: F, t121003: F, t121006: F, t121009: F, t121019: F, t121132: F, t121134: F, t121136: F, t121138: F, t26969: F, t32674: F, t32676: F, t33746: F, t7171: F, t8690: F) -> (F,) {
    let t124924 = 3.0 * t26969 * t8690 + 3.0 * t33746 * t7171 - t120995 - t120998 - t121003 - t121006 - t121009 - t121019 + t121132 - t121134 - t121136 - t121138 - t32674 - t32676;
    (t124924,)
}
