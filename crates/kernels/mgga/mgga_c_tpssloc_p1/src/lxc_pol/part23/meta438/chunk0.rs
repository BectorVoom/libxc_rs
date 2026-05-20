//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1281/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281<F: Float>(t15338: F, t18409: F, t3447: F, t20217: F, t3450: F, t18469: F, t52059: F, t4904: F, t64763: F, t18532: F, t4889: F, t1174: F, t135: F, t22040: F) -> (F, F, F, F, F, F) {
    let t73395 = t3447 * t15338 * t18409;
    let t73405 = t3450 * t20217;
    let t73417 = t3447 * t52059 * t18469;
    let t73420 = t3447 * t64763 * t4904;
    let t73424 = t4889 * t18532;
    let t73427 = t1174 * t135 * t22040;
    (t73395, t73405, t73417, t73420, t73424, t73427)
}
