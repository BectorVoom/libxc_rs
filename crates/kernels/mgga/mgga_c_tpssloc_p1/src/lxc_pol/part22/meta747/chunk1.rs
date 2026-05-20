//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2489/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2489<F: Float>(t2979: F, t43248: F, t50259: F, t50263: F, t62657: F, t62660: F, t62663: F, t62666: F, t62682: F, t62687: F, t68462: F, t68481: F, t973: F) -> F {
    let t70837 = -t62657 / F::new(36.0) + t62660 / F::new(108.0) - t62663 / F::new(144.0) + t62666 / F::new(216.0) + t973 * t2979 * t68481 / F::new(6.0) - t973 * t2979 * t68462 / F::new(12.0) + t50259 - t50263 + t62682 / F::new(1152.0) - t62687 / F::new(576.0) - t43248 / F::new(1944.0);
    t70837
}
