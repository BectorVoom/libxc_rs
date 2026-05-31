//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2150/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2150<F: Float>(t11708: F, t15502: F, t15506: F, t10469: F, t1720: F, t10471: F, t11737: F, t11791: F, t5005: F, t11677: F, t15027: F, t3575: F, t373: F, t470: F, t493: F) -> (F, F, F, F, F, F, F, F) {
    let t52810 = t11708 * t15502;
    let t52813 = t11708 * t15506;
    let t52834 = t1720 * t10469;
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    let t52872 = t5005 * t11791;
    let t52873 = t52872 / F::cast_from(6912.0_f64);
    let t52879 = t15027 * t11677;
    let t52893 = t470 * t493 * t3575 * t373;
    (t52810, t52813, t52834, t52835, t52836, t52873, t52879, t52893)
}
