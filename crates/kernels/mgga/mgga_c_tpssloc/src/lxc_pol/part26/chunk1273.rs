//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1273/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1273<F: Float>(t1888: F, t232: F, t6646: F, t81672: F, t2627: F, t6624: F, t10016: F, t1909: F, t22993: F, t23009: F, t2617: F, t2633: F, t812: F, t81623: F, t81627: F, t81630: F, t81633: F, t81637: F, t81642: F, t81645: F, t81648: F, t81653: F, t81656: F, t81661: F, t81667: F, t81670: F) -> F {
    let t81675 = t1888 * t6646 * t81672 * t232;
    let t81679 = t2627 * t6624;
    let t81683 = t10016 * t1909 + F::new(0.23029076935875170111e0) * t81623 - F::new(0.16449340668482264365e-1) * t81627 + F::new(0.24674011002723396548e-1) * t81630 - F::new(0.38381794893125283518e0) * t81633 - F::new(0.49348022005446793095e-1) * t81637 - F::new(0.74022033008170189643e-1) * t81642 + F::new(0.49348022005446793095e-1) * t81645 - F::new(0.24674011002723396548e-1) * t81648 - F::new(0.49348022005446793095e-1) * t81653 + F::new(0.49348022005446793095e-1) * t81656 - F::new(0.49348022005446793095e-1) * t81661 - F::new(6.0) * t2617 * t22993 - F::new(0.24674011002723396548e-1) * t81667 + F::new(0.24674011002723396547e-1) * t81670 - F::new(0.82246703342411321825e-2) * t81675 + F::new(6.0) * t2617 * t23009 + F::new(6.0) * t812 * t81679 * t2633;
    t81683
}
