//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1156/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1156<F: Float>(t1266: F, t8230: F, t1849: F, t8143: F, t30180: F, t510: F, t2180: F, t5107: F, t1393: F, t1268: F, t12725: F, t19456: F, t2181: F, t2183: F, t2314: F, t26114: F, t26117: F, t4028: F, t4034: F, t652: F, t7458: F, t8124: F, t8144: F, t8148: F, t8221: F, t8231: F) -> (F, F, F, F, F, F) {
    let t30195 = t1266 * t8230;
    let t30201 = t8143 * t1849;
    let t30203 = t510 * t30180;
    let t30209 = t5107 * t2180;
    let t30211 = t8230 * t1393;
    let t30215 = t1268 * t30201 + t1268 * t30211 + t12725 * t2183 - t19456 * t2181 + t19456 * t2183 + t2183 * t26114 + t2183 * t26117 - t2314 * t8221 - t30195 * t652 - t30203 * t652 - t30209 * t652 - t4028 * t8124 + t4028 * t8148 - t4034 * t8221 - t4034 * t8231 - t7458 * t8144;
    (t30195, t30201, t30203, t30209, t30211, t30215)
}
