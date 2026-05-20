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
    let t53258 = t53238 * t11724 / F::new(512.0) - t45108 / F::new(1152.0) - t45112 - t11665 * t15459 / F::new(1536.0) - t11665 * t15463 / F::new(768.0) + t53246 / F::new(432.0) + F::new(5.0) / F::new(6912.0) * t45126 - F::new(7.0) / F::new(216.0) * t3447 * t53249 * t45971 - t45148 / F::new(1536.0) + F::new(5.0) / F::new(4608.0) * t5005 * t11774 - F::new(5.0) / F::new(1728.0) * t3490 * t15455;
    t53258
}
