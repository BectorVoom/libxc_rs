//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1324/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1324<F: Float>(t18296: F, t6243: F, t19570: F, t508: F, t1760: F, t5709: F, t19620: F, t26009: F, t4478: F, t19602: F, t5706: F, t2049: F, t6076: F, t77: F, t1317: F, t5506: F) -> (F, F, F, F, F, F) {
    let t65134 = 2.0 * t6243 * t18296;
    let t65135 = t508 * t19570;
    let t65138 = 6.0 * t1760 * t65135 * t5709;
    let t65141 = 12.0 * t19620 * t26009 * t4478;
    let t65143 = 2.0 * t5706 * t19602;
    let t65152 = t77 * t6076 * t2049;
    let t65157 = t5506 * t1317;
    (t65134, t65138, t65141, t65143, t65152, t65157)
}
