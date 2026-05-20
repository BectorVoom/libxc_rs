//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2479/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479<F: Float>(t135: F, t21561: F, t973: F, t10390: F, t10413: F, t14207: F, t17712: F, t17732: F, t17984: F, t21526: F, t21566: F, t3071: F, t3130: F, t369: F, t378: F, t42505: F, t4347: F, t4582: F, t50265: F, t5869: F, t5878: F, t62164: F, t62177: F, t62183: F, t68: F, t70012: F) -> F {
    let t70497 = t973 * t135 * t21561;
    let t70509 = t3130 * t4582 * t17712 * t17732 / F::new(512.0) - F::new(3.0) / F::new(512.0) * t50265 * t17984 - t62164 / F::new(1536.0) - t62177 / F::new(4608.0) + t62183 / F::new(4608.0) - t42505 * t21526 / F::new(144.0) + t14207 * t5869 / F::new(1024.0) + t70497 / F::new(144.0) + t70012 * t68 * t369 * t378 / F::new(3072.0) - t10413 * t3071 * t5878 * t4347 / F::new(1536.0) + t10390 * t21566 / F::new(1536.0);
    t70509
}
