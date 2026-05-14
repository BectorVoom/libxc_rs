//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1258/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1258<F: Float>(t1981: F, t3432: F, t10292: F, t582: F, t19345: F, t5502: F, t18351: F, t6086: F, t6090: F, t619: F, t1678: F, t19424: F, t7690: F, t42178: F, t5486: F, t18341: F) -> (F, F, F, F, F, F, F, F) {
    let t65175 = t1981 * t3432;
    let t65189 = t10292 * t582;
    let t65198 = t5502 * t19345;
    let t65205 = t6086 * t18351;
    let t65208 = t6090 * t619;
    let t65209 = t1678 * t65208;
    let t65258 = t7690 * t19424;
    let t65336 = t42178 * t5486;
    let t65339 = t10292 * t18341;
    (t65175, t65189, t65198, t65205, t65209, t65258, t65336, t65339)
}
