//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1379/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1379<F: Float>(t114795: F, t114811: F, t114815: F, t118640: F, t118810: F, t118814: F, t118825: F, t121444: F, t121448: F, t121451: F, t121454: F, t121457: F, t1528: F, t25170: F, t2597: F, t26729: F, t33399: F, t866: F) -> F {
    let t121462 = F::cast_from(0.41123351671205660912e-2_f64) * t114795 - F::cast_from(0.82246703342411321825e-2_f64) * t121444 + F::cast_from(0.16449340668482264365e-1_f64) * t121448 - t114811 * t1528 - F::new(6.0) * t121451 * t25170 - t121454 * t866 - t118810 - t114815 - F::cast_from(0.82246703342411321825e-2_f64) * t121457 - t2597 * t33399 - F::new(6.0) * t118640 * t26729 + t118814 + t118825;
    t121462
}
