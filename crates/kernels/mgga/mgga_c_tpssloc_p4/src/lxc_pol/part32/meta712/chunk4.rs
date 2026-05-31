//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2237/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2237<F: Float>(t22992: F, t25269: F, t25297: F, t4166: F, t4182: F, t4281: F, t5612: F, t812: F, t81615: F, t87166: F, t87521: F, t87523: F, t87534: F, t92543: F, t98502: F, t98505: F, t98513: F, t98516: F, t98520: F, t98524: F, t98530: F, t98534: F) -> F {
    let t98536 = F::cast_from(0.9869604401089358619e-1_f64) * t98502 + t87166 + F::cast_from(0.82246703342411321824e-2_f64) * t81615 + F::cast_from(0.38381794893125283518e-1_f64) * t98505 - F::cast_from(2.0_f64) * t4166 * t25269 - F::cast_from(2.0_f64) * t4166 * t25297 + F::cast_from(0.49348022005446793095e-1_f64) * t98513 - F::cast_from(0.24674011002723396548e-1_f64) * t98516 - F::cast_from(0.3289868133696452873e-1_f64) * t98520 + t92543 - t812 * t22992 * t5612 - t87521 + F::cast_from(4.0_f64) * t4281 * t98524 * t4182 + t87523 - F::cast_from(0.82246703342411321825e-2_f64) * t98530 + t87534 + F::cast_from(0.16449340668482264365e-1_f64) * t98534;
    t98536
}
