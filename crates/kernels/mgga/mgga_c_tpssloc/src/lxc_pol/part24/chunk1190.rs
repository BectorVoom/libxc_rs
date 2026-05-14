//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1190/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1190<F: Float>(t6552: F, t6637: F, t776: F, t81658: F, t1888: F, t232: F, t40955: F, t6646: F, t23110: F, t23176: F, t23185: F, t252: F, t9660: F, t2627: F, t6624: F, t10016: F, t1909: F, t22993: F, t23009: F, t2617: F, t2633: F, t812: F, t81623: F, t81627: F, t81630: F, t81633: F, t81637: F, t81642: F, t81645: F, t81648: F, t81653: F, t81656: F) -> (F, F) {
    let t81661 = t6552 * t6637 * t81658 * t776;
    let t81667 = t1888 * t6646 * t40955 * t232;
    let t81670 = t23185 * t23110 * t23176;
    let t81672 = t252 * t9660;
    let t81675 = t1888 * t6646 * t81672 * t232;
    let t81679 = t2627 * t6624;
    let t81683 = t10016 * t1909 + 0.23029076935875170111e0 * t81623 - 0.16449340668482264365e-1 * t81627 + 0.24674011002723396548e-1 * t81630 - 0.38381794893125283518e0 * t81633 - 0.49348022005446793095e-1 * t81637 - 0.74022033008170189643e-1 * t81642 + 0.49348022005446793095e-1 * t81645 - 0.24674011002723396548e-1 * t81648 - 0.49348022005446793095e-1 * t81653 + 0.49348022005446793095e-1 * t81656 - 0.49348022005446793095e-1 * t81661 - 6.0 * t2617 * t22993 - 0.24674011002723396548e-1 * t81667 + 0.24674011002723396547e-1 * t81670 - 0.82246703342411321825e-2 * t81675 + 6.0 * t2617 * t23009 + 6.0 * t812 * t81679 * t2633;
    (t81672, t81683)
}
