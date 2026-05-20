//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2306/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306<F: Float>(t15590: F, t7338: F, t27614: F, t3572: F, t27617: F, t3523: F, t1218: F, t15531: F, t15535: F, t15622: F, t15627: F, t15631: F, t15637: F, t24729: F, t24733: F, t4984: F, t86120: F, t86146: F, t86164: F, t86167: F, t86171: F) -> F {
    let t95238 = t15590 * t7338;
    let t95242 = t27614 * t3572 / F::new(1152.0);
    let t95244 = t27617 * t3523 / F::new(1728.0);
    let t95260 = t95238 * t1218 / F::new(768.0) + t95242 - t95244 + t24729 * t15622 / F::new(768.0) + t86146 * t15627 / F::new(256.0) - t86164 * t15631 / F::new(256.0) - t86167 * t4984 / F::new(768.0) - t24733 * t15637 / F::new(768.0) - t24733 * t15531 / F::new(1536.0) + t86171 * t15535 / F::new(1536.0) + F::new(5.0) / F::new(10368.0) * t86120;
    t95260
}
