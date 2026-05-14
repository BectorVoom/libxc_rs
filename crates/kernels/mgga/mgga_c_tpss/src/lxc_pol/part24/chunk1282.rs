//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1282/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1282<F: Float>(t10292: F, t1290: F, t19345: F, t6086: F, t21128: F, t619: F, t77: F, t1317: F, t6090: F, t1678: F, t21132: F, t1679: F, t4626: F, t1981: F, t4580: F, t18345: F, t18350: F, t18352: F, t19342: F, t19346: F, t19349: F, t65189: F, t65205: F, t65209: F, t65258: F, t65406: F) -> (F,) {
    let t69186 = t10292 * t1290;
    let t69191 = t6086 * t19345;
    let t69195 = t77 * t21128 * t619;
    let t69198 = t6090 * t1317;
    let t69199 = t1678 * t69198;
    let t69203 = t77 * t21132 * t619;
    let t69206 = t1679 * t4626;
    let t69207 = t1678 * t69206;
    let t69210 = t1981 * t4580;
    let t69217 = -10.0 * t65406 * t19342 - 10.0 / 3.0 * t65189 * t19346 - 10.0 / 3.0 * t69186 * t18352 - 10.0 * t65258 * t19342 - 10.0 / 3.0 * t18350 * t69191 - 10.0 * t18345 * t69195 - 10.0 / 3.0 * t18350 * t69199 - 5.0 * t18345 * t69203 - 5.0 / 3.0 * t18350 * t69207 - 5.0 / 3.0 * t69210 * t18352 - 10.0 / 3.0 * t19349 * t65205 - 10.0 / 3.0 * t19349 * t65209;
    (t69217,)
}
