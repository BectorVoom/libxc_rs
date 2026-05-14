//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1171/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1171<F: Float>(t1268: F, t12725: F, t1393: F, t19451: F, t2181: F, t2183: F, t26114: F, t26117: F, t28002: F, t28007: F, t30186: F, t30189: F, t30203: F, t30211: F, t30424: F, t30428: F, t4028: F, t5113: F, t55943: F, t7458: F, t75560: F, t7676: F, t8144: F, t8148: F, t8150: F, t8231: F, t8235: F, t96657: F, t96709: F) -> (F,) {
    let t111546 = 2.0 * t1268 * t1393 * t30424 - 4.0 * t12725 * t8231 - 2.0 * t19451 * t8144 + 2.0 * t19451 * t8148 + 2.0 * t19451 * t8150 - 2.0 * t2181 * t55943 + 2.0 * t2183 * t75560 + 2.0 * t2183 * t96657 + 2.0 * t2183 * t96709 - 4.0 * t26114 * t8231 + 4.0 * t26114 * t8235 + 4.0 * t26117 * t8235 + 4.0 * t28002 * t8150 + 2.0 * t28007 * t8148 + 4.0 * t30186 * t4028 + 4.0 * t30186 * t7676 - 4.0 * t30189 * t7458 - 4.0 * t30203 * t7458 + 4.0 * t30211 * t7676 + 4.0 * t30428 * t5113;
    (t111546,)
}
