//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1317/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1317<F: Float>(t10987: F, t135: F, t973: F, t10394: F, t10405: F, t10408: F, t10415: F, t10937: F, t10944: F, t10957: F, t10988: F, t2771: F, t2780: F, t2960: F, t3064: F, t3070: F, t3071: F, t3073: F, t3121: F, t3134: F, t42505: F, t42508: F, t42511: F, t42514: F, t42518: F, t42522: F) -> F {
    let t42530 = t973 * t135 * t10987;
    let t42540 = -t10937 * t10394 / F::cast_from(72.0_f64) - t42505 * t10405 / F::cast_from(36.0_f64) + t42508 * t10415 / F::cast_from(72.0_f64) + t42511 * t3073 / F::cast_from(384.0_f64) - t42514 / F::cast_from(108.0_f64) + F::cast_from(95.0_f64) / F::cast_from(1296.0_f64) * t10957 * t3064 - F::cast_from(5.0_f64) / F::cast_from(324.0_f64) * t42518 + F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t42522 * t3134 - t2960 * t10988 / F::cast_from(27.0_f64) - F::cast_from(28.0_f64) / F::cast_from(243.0_f64) * t2960 * t10944 + t42530 / F::cast_from(216.0_f64) + t3070 * t3071 * t3121 * t2780 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3070 * t10408 * t3121 * t2771;
    t42540
}
