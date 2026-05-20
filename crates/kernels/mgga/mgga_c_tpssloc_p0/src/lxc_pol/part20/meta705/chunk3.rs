//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2681/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681<F: Float>(t12178: F, t12255: F, t12303: F, t12371: F, t16305: F, t16311: F, t16312: F, t19735: F, t19876: F, t3803: F, t3805: F, t3807: F, t40168: F, t40285: F, t40293: F, t40295: F, t5246: F, t5301: F, t54258: F, t54585: F, t54591: F, t54607: F, t54609: F, t54612: F, t54614: F) -> F {
    let t54625 = F::new(7.0) / F::new(1536.0) * t54585 - t5246 * t16305 * t19735 * t16312 / F::new(64.0) - t5246 * t16305 * t16311 * t54591 / F::new(128.0) + t3803 * t16305 * t54258 * t3807 / F::new(256.0) + F::new(7.0) / F::new(384.0) * t40285 - F::new(119.0) / F::new(576.0) * t40293 + F::new(7.0) / F::new(1152.0) * t40295 - t5246 * t3805 * t5301 * t12255 / F::new(128.0) - F::new(7.0) / F::new(384.0) * t54607 - F::new(7.0) / F::new(1536.0) * t54609 + t54612 - F::new(15.0) / F::new(128.0) * t54614 * t40168 * t5301 * t12303 - t19876 * t12371 / F::new(128.0) + t3803 * t3805 * t5301 * t12178 / F::new(768.0);
    t54625
}
