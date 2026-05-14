//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1227/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1227<F: Float>(t21139: F, t5791: F, t20275: F, t6080: F, t1675: F, t21165: F, t5790: F, t21146: F, t6073: F, t1791: F, t21756: F, t5483: F, t5785: F, t67474: F, t67480: F, t67491: F, t67496: F, t69338: F, t69355: F) -> (F,) {
    let t71503 = t21139 * t5791;
    let t71505 = t6080 * t20275;
    let t71508 = t1675 * t5790 * t21165;
    let t71510 = t21146 * t5791;
    let t71512 = t6073 * t20275;
    let t71520 = -5.0 / 3.0 * t5785 * t69355 + 16.0 / 9.0 * t71503 + 32.0 / 9.0 * t71505 - t67474 - 8.0 / 9.0 * t71508 - 8.0 / 9.0 * t71510 - 16.0 / 9.0 * t71512 + t67480 + t67491 + 176.0 / 27.0 * t67496 + t5483 * t21756 / 3.0 + t1675 * t1791 * t69338 / 3.0;
    (t71520,)
}
