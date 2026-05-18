//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 719/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk719<F: Float>(t1912: F, t259: F, t2597: F, t2713: F, t6549: F, t6557: F, t6565: F, t6569: F, t6574: F, t6576: F, t6625: F, t6627: F, t6632: F, t6663: F, t855: F, t866: F) -> F {
    let t6665 = -t6549 - F::new(0.16449340668482264365e-1) * t6557 - t6565 + F::new(0.82246703342411321825e-2) * t6569 - F::new(0.82246703342411321825e-2) * t6574 + t6576 * t259 + t6625 * t259 - t6627 * t866 - t2597 * t1912 - t2713 * t1912 + F::new(2.0) * t855 * t6632 - t855 * t6663;
    t6665
}
