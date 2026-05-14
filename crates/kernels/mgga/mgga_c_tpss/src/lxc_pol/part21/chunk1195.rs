//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1195/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1195<F: Float>(t18532: F, t509: F, t1270: F, t1760: F, t5706: F, t5755: F, t1777: F, t507: F, t3245: F, t196: F, t197: F, t3174: F, t1779: F, t508: F, t1759: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18533 = t509 * t18532;
    let t18534 = t18533 * t1270;
    let t18535 = t1760 * t18534;
    let t18537 = 2.0 * t5706 * t5755;
    let t18538 = t507 * t1777;
    let t18539 = t1270 * t3245;
    let t18540 = t18538 * t18539;
    let t18542 = 6.0 * t1760 * t18540;
    let t18544 = t3174 * t196 * t197;
    let t18545 = t18544 * t1779;
    let t18546 = t197 * t508;
    let t18547 = t1759 * t18546;
    (t18533, t18534, t18535, t18537, t18539, t18540, t18542, t18544, t18545, t18546, t18547)
}
