//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1085/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1085<F: Float>(t343: F, t5836: F, t6734: F, t5842: F, t1941: F, t5904: F, t1011: F, t5872: F, t3131: F, t23512: F, t360: F, t23519: F, t5866: F, t68: F, t6744: F, t1935: F, t23419: F, t23469: F, t23510: F, t25639: F, t25642: F, t25683: F, t378: F, t5885: F, t5890: F, t5894: F, t5900: F, t5909: F, t6717: F, t6742: F, t6765: F, t7574: F, t7578: F, t7583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28557 = t5836 * t343;
    let t28558 = t28557 * t6734;
    let t28565 = t5842 * t343;
    let t28566 = t28565 * t6734;
    let t28572 = t5904 * t1941;
    let t28576 = t5872 * t1011;
    let t28577 = t28576 * t3131;
    let t28578 = t23512 * t28577;
    let t28581 = t28576 * t360;
    let t28582 = t23519 * t28581;
    let t28586 = t5866 * t68 * t360;
    let t28587 = t6744 * t28586;
    let t28592 = -t23469 - t6765 * t5900 / 1152.0 - t6717 * t5885 / 144.0 - 0.20186378047070195428e-3 * t7574 * t7578 - 0.10093189023535097714e-3 * t1935 * t28558 + t6717 * t5890 / 288.0 + t6717 * t5894 / 216.0 - 0.10093189023535097714e-3 * t1935 * t28566 + t23419 * t5909 / 1152.0 - 0.20186378047070195428e-3 * t25639 + t28572 * t378 / 1536.0 + 0.20186378047070195428e-3 * t25642 + 0.20186378047070195428e-3 * t23510 * t28578 - 0.10093189023535097714e-3 * t23510 * t28582 + 0.10093189023535097714e-3 * t6742 * t28587 + 0.20186378047070195428e-3 * t25683 * t7583;
    (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582, t28586, t28587, t28592)
}
