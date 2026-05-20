//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1000/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1000<F: Float>(t11620: F, t1246: F, t1235: F, t3507: F, t3625: F, t1155: F, t3375: F, t3396: F, t1164: F, t11128: F, t11133: F, t11179: F, t11182: F, t11184: F, t11187: F, t11405: F, t11409: F, t11426: F, t11429: F) -> (F, F, F, F, F, F, F) {
    let t11621 = t11620 * t1246;
    let t11624 = t1235 * t3507;
    let t11625 = t11624 * t3625;
    let t11628 = t3375 * t1155;
    let t11629 = t11628 * t3396;
    let t11631 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t11629;
    let t11632 = -t11426 + t11429 - t11405 + t11409 + t11631 - t11128 - t11133 + t11179 + t11182 + t11184 + t11187;
    (t11621, t11624, t11625, t11628, t11629, t11631, t11632)
}
