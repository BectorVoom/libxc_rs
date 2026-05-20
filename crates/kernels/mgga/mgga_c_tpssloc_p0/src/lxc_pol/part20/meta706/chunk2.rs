//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2692/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2692<F: Float>(t54811: F, t119: F, t12407: F, t12429: F, t1315: F, t16242: F, t16248: F, t16265: F, t16364: F, t16383: F, t210: F, t3803: F, t3805: F, t3851: F, t3856: F, t40443: F, t40449: F, t5248: F, t53856: F, t54786: F, t54787: F, t54793: F, t54801: F) -> F {
    let t54812 = F::new(119.0) / F::new(2304.0) * t54811;
    let t54813 = -t12429 * t16265 / F::new(1024.0) + t3803 * t3805 * t16242 * t12407 / F::new(256.0) + t12429 * t16248 / F::new(256.0) + t54786 + F::new(7.0) / F::new(48.0) * t54787 - t1315 * t210 * t119 * t53856 / F::new(48.0) - F::new(595.0) / F::new(10368.0) * t54793 + F::new(119.0) / F::new(4608.0) * t40443 + t40449 - t3803 * t5248 * t16242 * t3851 / F::new(1024.0) - F::new(7.0) / F::new(384.0) * t54801 + t12429 * t16383 / F::new(256.0) + t3803 * t3805 * t16364 * t3856 / F::new(256.0) + t54812;
    t54813
}
