//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2307/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307<F: Float>(t15437: F, t24728: F, t24732: F, t4965: F, t7344: F, t1232: F, t1737: F, t27604: F, t27614: F, t27617: F, t3496: F, t3511: F, t3518: F, t3527: F, t3531: F, t86122: F, t86124: F, t86126: F, t86136: F) -> F {
    let t95270 = t15437 * t24728;
    let t95273 = t15437 * t24732;
    let t95276 = t4965 * t7344;
    let t95285 = t86122 / F::new(1152.0) - t86124 / F::new(1728.0) - t86136 / F::new(1728.0) + t27604 * t3527 / F::new(432.0) + t27604 * t3531 / F::new(216.0) + t27614 * t3496 / F::new(1536.0) + t95270 * t3511 / F::new(768.0) - t95273 * t3518 / F::new(1536.0) - t95276 * t1232 / F::new(1152.0) - t27617 * t3527 / F::new(2304.0) - t27617 * t3531 / F::new(1152.0) + t86126 * t1737 / F::new(1536.0);
    t95285
}
