//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1283/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1283<F: Float>(t4626: F, t615: F, t77: F, t13447: F, t84: F, t1677: F, t1981: F, t4573: F, t1289: F, t578: F, t3431: F, t1976: F, t4580: F, t13330: F, t13336: F, t1680: F, t18335: F, t18342: F, t19342: F, t21133: F, t21139: F, t25122: F, t5487: F, t5489: F, t5503: F, t5507: F, t7690: F) -> (F,) {
    let t69228 = t77 * t615 * t4626;
    let t69232 = t77 * t84 * t13447;
    let t69236 = t1981 * t4573 * t1677;
    let t69240 = t578 * t1289 * t1677;
    let t69242 = t77 * t84 * t3431;
    let t69245 = t1976 * t4580;
    let t69248 = t578 * t13330;
    let t69251 = t578 * t13336;
    let t69258 = 20.0 * t7690 * t25122 * t19342 + 5.0 / 6.0 * t18335 * t21133 + 5.0 / 6.0 * t18342 * t21133 + 5.0 / 6.0 * t5487 * t69228 + 5.0 / 6.0 * t5487 * t69232 - 5.0 / 3.0 * t69236 * t5489 + 2.0 / 3.0 * t69240 * t69242 + t69245 * t1680 / 3.0 + t69248 * t1680 / 3.0 + t69251 * t1680 / 3.0 + t21139 * t5503 / 3.0 + t21139 * t5507 / 3.0;
    (t69258,)
}
