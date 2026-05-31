//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1375/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1375<F: Float>(t118639: F, t118650: F, t118654: F, t118662: F, t118664: F, t118667: F, t118672: F, t121399: F, t121403: F, t121405: F, t121409: F, t2054: F, t2597: F, t33452: F, t866: F, t87837: F) -> F {
    let t121411 = t118639 + F::cast_from(2.0_f64) * t2597 * t33452 + t118650 + F::cast_from(0.41123351671205660912e-2_f64) * t121399 - F::cast_from(0.16449340668482264365e-1_f64) * t121403 + t118654 - t118662 - t118664 + t118667 - t121405 * t866 - t87837 * t2054 - F::cast_from(0.82246703342411321825e-2_f64) * t121409 - t118672;
    t121411
}
