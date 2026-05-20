//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2658/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2658<F: Float>(t40138: F, t5303: F, t12283: F, t16366: F, t16308: F, t1352: F, t16153: F, t16224: F, t16242: F, t16311: F, t3803: F, t3805: F, t3856: F, t40052: F, t40054: F, t40060: F, t40065: F, t40079: F, t40081: F, t40083: F, t40178: F, t5246: F, t5248: F, t5249: F, t54013: F, t54015: F) -> F {
    let t54220 = t40138 * t5303;
    let t54222 = t12283 * t16366;
    let t54237 = t12283 * t16308;
    let t54245 = -t3803 * t5248 * t16242 * t3856 / F::new(1024.0) - F::new(7.0) / F::new(192.0) * t54220 - F::new(7.0) / F::new(192.0) * t54222 + t3803 * t3805 * t5249 * t40178 / F::new(768.0) - F::new(5.0) / F::new(256.0) * t3803 * t16224 * t16153 * t1352 + F::new(35.0) / F::new(384.0) * t40052 + F::new(3.0) / F::new(512.0) * t5246 * t54013 * t16311 * t54015 - F::new(7.0) / F::new(192.0) * t54237 + F::new(7.0) / F::new(384.0) * t40054 + F::new(595.0) / F::new(864.0) * t40060 - F::new(119.0) / F::new(1152.0) * t40065 + F::new(595.0) / F::new(1152.0) * t40079 + F::new(35.0) / F::new(192.0) * t40081 - F::new(35.0) / F::new(384.0) * t40083;
    t54245
}
