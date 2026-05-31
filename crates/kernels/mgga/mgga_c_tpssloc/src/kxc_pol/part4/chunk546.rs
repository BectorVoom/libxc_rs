//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 546/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk546<F: Float>(t814: F, t852: F, t261: F, t1878: F, t268: F, t271: F, t690: F, t885: F) -> (F, F, F, F, F, F) {
    let t2732 = t814 * t852;
    let t2751 = t261 * t261;
    let t2752 = F::cast_from(1.0_f64) / t2751;
    let t2764 = t268 * t1878 * t271;
    let t2765 = F::cast_from(0.23744444444444444444e-1_f64) * t2764;
    let t2766 = t690 * t885;
    (t2732, t2751, t2752, t2764, t2765, t2766)
}
