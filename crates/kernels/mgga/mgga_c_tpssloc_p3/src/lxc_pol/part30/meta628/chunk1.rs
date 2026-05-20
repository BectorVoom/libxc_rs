//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2033/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2033<F: Float>(t849: F, t87340: F, t1516: F, t81763: F, t23083: F, t25094: F, t23046: F, t4184: F, t812: F, t836: F, t242: F, t81816: F) -> (F, F, F, F, F) {
    let t87341 = t87340 * t849;
    let t87342 = F::new(7.0) / F::new(288.0) * t87341;
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87348 = F::cast_from(0.56521858531796547196e-2_f64) * t87347;
    let t87363 = t812 * t23046 * t836 * t4184;
    let t87364 = F::new(7.0) / F::new(576.0) * t87363;
    let t87368 = t812 * t81816 * t242;
    (t87342, t87345, t87348, t87364, t87368)
}
