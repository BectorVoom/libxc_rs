//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1155/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1155<F: Float>(t30180: F, t574: F, t2180: F, t5361: F, t1774: F, t8143: F, t1268: F, t12725: F, t2181: F, t2314: F, t26114: F, t26179: F, t4028: F, t5113: F, t652: F, t7458: F, t7676: F, t8124: F, t8144: F, t8148: F, t8150: F, t8231: F, t8235: F, t8237: F) -> (F, F, F, F) {
    let t30181 = t30180 * t574;
    let t30186 = t2180 * t5361;
    let t30189 = t1774 * t8143;
    let t30192 = t1268 * t30181 + t1268 * t30186 - t12725 * t2181 - t2181 * t26114 - t2181 * t26179 - t2314 * t8231 + t2314 * t8235 + t2314 * t8237 - t30189 * t652 - t4028 * t8144 + t4028 * t8150 + t5113 * t8235 + t5113 * t8237 - t7458 * t8124 + t7676 * t8148 + t7676 * t8150;
    (t30181, t30186, t30189, t30192)
}
