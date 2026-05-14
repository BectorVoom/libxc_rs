//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1169/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1169<F: Float>(t11539: F, t1174: F, t22055: F, t18454: F, t4889: F, t22059: F, t3431: F, t18529: F, t135: F, t22034: F, t15338: F, t18409: F, t3447: F, t20217: F, t3450: F, t18469: F, t52059: F) -> (F, F, F, F, F, F, F, F) {
    let t73307 = t1174 * t11539 * t22055;
    let t73314 = t4889 * t18454;
    let t73330 = t1174 * t3431 * t22059;
    let t73386 = t4889 * t18529;
    let t73389 = t1174 * t135 * t22034;
    let t73395 = t3447 * t15338 * t18409;
    let t73405 = t3450 * t20217;
    let t73417 = t3447 * t52059 * t18469;
    (t73307, t73314, t73330, t73386, t73389, t73395, t73405, t73417)
}
