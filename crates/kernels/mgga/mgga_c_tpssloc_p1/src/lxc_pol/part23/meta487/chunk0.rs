//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1493/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1493<F: Float>(t54325: F, t56168: F, t54380: F, t54382: F, t20067: F, t20077: F, t39356: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t5126: F, t6330: F) -> (F, F, F, F, F) {
    let t79896 = F::cast_from(0.22787578869697033845e-2_f64) * t54325;
    let t79897 = F::cast_from(0.70178683471615754484e1_f64) * t56168;
    let t79898 = F::cast_from(0.65061487801810439052e-1_f64) * t54380;
    let t79899 = F::cast_from(0.19263893255070628431e1_f64) * t54382;
    let t79903 = F::new(36.0) * t20067 * t5126 * t6330 - F::new(36.0) * t20077 * t5126 * t6330 + t39356 + t39360 + t39364 + t39373 - t39384 + t39393 - t39397 - t39400 + t39408 - t79896 + t79897 + t79898 + t79899;
    (t79896, t79897, t79898, t79899, t79903)
}
