//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1206/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1206<F: Float>(t12022: F, t12027: F, t1375: F, t1386: F, t2015: F, t2016: F, t22653: F, t3882: F, t39910: F, t40591: F, t6958: F, t80671: F, t80675: F, t80678: F, t80683: F, t80687: F, t80689: F, t80699: F) -> F {
    let t80702 = -F::new(0.15626873635058151147e0) * t80671 - F::new(0.82246703342411321825e-2) * t80675 + F::new(0.14804406601634037928e0) * t80678 - F::new(0.74022033008170189643e-1) * t80683 - t39910 * t2016 - F::new(0.24674011002723396548e-1) * t80687 + F::new(0.57572692339687925277e-1) * t80689 + F::new(6.0) * t6958 * t12027 + F::new(24.0) * t1375 * t40591 * t2015 * t12022 + F::new(12.0) * t3882 * t22653 - F::new(6.0) * t80699 * t1386;
    t80702
}
