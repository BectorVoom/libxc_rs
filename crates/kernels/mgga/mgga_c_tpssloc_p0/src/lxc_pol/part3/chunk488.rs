//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 488/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk488<F: Float>(t1742: F, t479: F, t471: F, t1230: F, t1653: F, t248: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t1737: F, t467: F, t488: F) -> (F, F, F, F) {
    let t1743 = t479 * t1742;
    let t1744 = t471 * t1743;
    let t1748 = t248 * t1230 * t1653;
    let t1751 = -t1706 * t467 / F::cast_from(36.0_f64) + t1195 - t1174 * t1726 / F::cast_from(288.0_f64) + t1731 * t488 / F::cast_from(3072.0_f64) + t1213 * t1737 / F::cast_from(3072.0_f64) - t1744 * t488 / F::cast_from(576.0_f64) + t1224 - t1227 * t1748 / F::cast_from(4608.0_f64);
    (t1743, t1744, t1748, t1751)
}
