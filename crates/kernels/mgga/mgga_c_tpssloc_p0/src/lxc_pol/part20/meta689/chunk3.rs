//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2614/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614<F: Float>(t1227: F, t14706: F, t248: F, t3521: F, t11814: F, t4997: F, t15492: F, t3536: F, t11781: F, t15594: F, t1748: F, t3531: F, t3578: F, t44918: F, t45015: F, t45020: F, t45027: F, t45044: F, t5005: F, t52236: F, t52893: F) -> F {
    let t53114 = t1227 * t248 * t3521 * t14706;
    let t53116 = t11814 * t4997;
    let t53118 = t3536 * t15492;
    let t53129 = -t45015 / F::new(1152.0) + t45020 / F::new(3456.0) - F::new(5.0) / F::new(5184.0) * t5005 * t11781 - t53114 / F::new(2304.0) + t53116 / F::new(1536.0) + t53118 / F::new(768.0) - t44918 * t1748 / F::new(4608.0) - t15594 * t3531 / F::new(768.0) - t45027 / F::new(1152.0) - t52893 * t3578 * t52236 / F::new(256.0) - F::new(5.0) / F::new(1296.0) * t45044;
    t53129
}
