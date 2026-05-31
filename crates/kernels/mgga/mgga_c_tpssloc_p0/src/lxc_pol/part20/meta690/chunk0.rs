//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2618/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618<F: Float>(t11718: F, t52835: F, t11797: F, t5024: F, t11147: F, t15394: F, t11665: F, t11724: F, t11774: F, t15455: F, t15459: F, t15463: F, t3447: F, t3490: F, t45108: F, t45112: F, t45126: F, t45148: F, t45971: F, t5005: F) -> F {
    let t53238 = t52835 * t11718;
    let t53246 = t5024 * t11797;
    let t53249 = t15394 * t11147;
    let t53258 = t53238 * t11724 / F::cast_from(512.0_f64) - t45108 / F::cast_from(1152.0_f64) - t45112 - t11665 * t15459 / F::cast_from(1536.0_f64) - t11665 * t15463 / F::cast_from(768.0_f64) + t53246 / F::cast_from(432.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t45126 - F::cast_from(7.0_f64) / F::cast_from(216.0_f64) * t3447 * t53249 * t45971 - t45148 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t5005 * t11774 - F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t3490 * t15455;
    t53258
}
