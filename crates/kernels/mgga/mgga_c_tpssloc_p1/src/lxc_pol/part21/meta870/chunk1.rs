//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3195/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3195<F: Float>(t27524: F, t607: F, t1215: F, t6224: F, t1227: F, t13969: F, t18954: F, t11709: F, t15617: F, t15702: F, t15708: F, t15709: F, t15740: F, t15750: F, t18236: F, t18397: F, t18948: F, t19002: F, t3247: F, t3508: F, t3577: F, t3578: F, t45112: F, t45119: F, t45134: F, t45162: F, t5005: F, t53220: F, t53246: F, t66360: F, t66363: F, t66372: F, t66374: F, t66378: F) -> (F, F) {
    let t66380 = t27524 * t607;
    let t66388 = t6224 * t1215;
    let t66398 = t1227 * t13969 * t18954;
    let t66400 = -t53220 / F::new(384.0) - t5005 * t15617 / F::new(384.0) - t66360 / F::new(1728.0) - t66363 / F::new(3456.0) - t3577 * t3578 * t18236 * t15708 / F::new(1152.0) - t45112 + t11709 * t18948 / F::new(384.0) + t66372 * t66374 * t15709 / F::new(576.0) - t66378 * t66374 * t3508 * t3247 * t66380 / F::new(288.0) + t53246 / F::new(648.0) + F::new(5.0) / F::new(3456.0) * t15740 * t15750 - t45119 * t3578 * t66388 * t15702 / F::new(2304.0) - t45162 * t19002 / F::new(576.0) + t45134 * t18397 / F::new(1152.0) - F::new(5.0) / F::new(3888.0) * t66398;
    (t66380, t66400)
}
