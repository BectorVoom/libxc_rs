//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2672/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672<F: Float>(t54462: F, t39851: F, t54467: F, t57227: F, t57229: F, t57235: F, t40224: F, t40230: F, t54459: F, t54461: F, t54465: F, t54466: F, t54470: F, t54472: F, t54473: F, t54475: F, t54478: F) -> (F, F, F, F, F, F, F) {
    let t74499 = F::new(360.0) * t54462;
    let t74500 = F::new(12.0) * t39851;
    let t74501 = F::cast_from(0.30762056574649219972e4_f64) * t54467;
    let t74502 = F::new(12.0) * t57227;
    let t74503 = F::new(12.0) * t57229;
    let t74504 = F::cast_from(0.32530743900905219526e-1_f64) * t57235;
    let t74505 = t54459 - t54461 - t74499 - t74500 - t54465 + t54466 - t74501 - t54470 - t54472 + t40224 + t54473 - t74502 - t74503 - t54475 - t40230 + t74504 - t54478;
    (t74499, t74500, t74501, t74502, t74503, t74504, t74505)
}
