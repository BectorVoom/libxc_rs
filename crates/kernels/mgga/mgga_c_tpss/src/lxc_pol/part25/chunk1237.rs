//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1237/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1237<F: Float>(t21805: F, t219: F, t1265: F, t1266: F, t13850: F, t1656: F, t1838: F, t18483: F, t18490: F, t18496: F, t18950: F, t19509: F, t19542: F, t19554: F, t20171: F, t20190: F, t20196: F, t20200: F, t20211: F, t21819: F, t21826: F, t21836: F, t21841: F, t21846: F, t21849: F, t520: F, t538: F, t5407: F, t5449: F, t5739: F, t5745: F, t5918: F, t5930: F, t60811: F, t6430: F, t65667: F, t67061: F, t69458: F, t71748: F, t71809: F) -> (F,) {
    let t71837 = t21805 * t219;
    let t71872 = 2.0 * t18483 * t21841 + param_beta * t71809 * t538 + 2.0 * t65667 * t6430 + 8.0 * t18496 * t20190 * t1656 * t19542 - 4.0 * t18496 * t67061 * t19554 - t71837 * t1266 + t5739 * t5745 * t5918 * t5407 * t520 + t5739 * t5745 * t1838 * t13850 * t520 + t5739 * t5745 * t71748 * t520 + t18483 * t21849 + 2.0 * t19509 * t20196 + 2.0 * t19509 * t20200 + t69458 * t5930 - 2.0 * t18483 * t21836 - t18950 * t5449 + t18483 * t21846 + 2.0 * t19509 * t20211 + 24.0 * t5739 * t60811 * t21819 * t1265 - 12.0 * t5739 * t18490 * t21826 * t1265 - 12.0 * t19509 * t20171;
    (t71872,)
}
