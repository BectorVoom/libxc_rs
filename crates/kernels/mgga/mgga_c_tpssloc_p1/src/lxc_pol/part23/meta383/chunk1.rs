//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1186/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1186<F: Float>(t3610: F, t52627: F, t1227: F, t1653: F, t248: F, t45293: F, t11677: F, t15245: F, t10469: F, t1720: F, t10471: F, t11737: F) -> (F, F, F, F, F, F) {
    let t52628 = t3610 * t52627;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52766 = t15245 * t11677;
    let t52834 = t1720 * t10469;
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    (t52628, t52680, t52766, t52834, t52835, t52836)
}
