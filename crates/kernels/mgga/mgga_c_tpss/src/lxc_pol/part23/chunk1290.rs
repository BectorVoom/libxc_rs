//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1290/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1290<F: Float>(t1777: F, t3245: F, t1760: F, t6245: F, t18289: F, t19604: F, t18544: F, t6277: F, t19605: F, t5706: F, t18296: F, t6243: F, t19570: F, t508: F, t5709: F, t19620: F, t26009: F, t4478: F) -> (F, F, F, F, F, F, F) {
    let t65122 = t3245 * t1777;
    let t65125 = 6.0 * t1760 * t65122 * t6245;
    let t65128 = 6.0 * t1760 * t18289 * t19604;
    let t65129 = t18544 * t6277;
    let t65131 = 6.0 * t5706 * t19605;
    let t65134 = 2.0 * t6243 * t18296;
    let t65135 = t508 * t19570;
    let t65138 = 6.0 * t1760 * t65135 * t5709;
    let t65141 = 12.0 * t19620 * t26009 * t4478;
    (t65125, t65128, t65129, t65131, t65134, t65138, t65141)
}
