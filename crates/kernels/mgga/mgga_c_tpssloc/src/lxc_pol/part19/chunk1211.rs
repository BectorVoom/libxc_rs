//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1211/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211<F: Float>(t10394: F, t10405: F, t10408: F, t10415: F, t10937: F, t10944: F, t10957: F, t10988: F, t2771: F, t2780: F, t2960: F, t3064: F, t3070: F, t3071: F, t3073: F, t3121: F, t3134: F, t42505: F, t42508: F, t42511: F, t42514: F, t42518: F, t42522: F, t42530: F) -> (F,) {
    let t42540 = -t10937 * t10394 / 72.0 - t42505 * t10405 / 36.0 + t42508 * t10415 / 72.0 + t42511 * t3073 / 384.0 - t42514 / 108.0 + 95.0 / 1296.0 * t10957 * t3064 - 5.0 / 324.0 * t42518 + 19.0 / 144.0 * t42522 * t3134 - t2960 * t10988 / 27.0 - 28.0 / 243.0 * t2960 * t10944 + t42530 / 216.0 + t3070 * t3071 * t3121 * t2780 / 768.0 + 5.0 / 2304.0 * t3070 * t10408 * t3121 * t2771;
    (t42540,)
}
