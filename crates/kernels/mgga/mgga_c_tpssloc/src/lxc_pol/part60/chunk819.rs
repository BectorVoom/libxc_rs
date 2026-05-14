//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 819/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk819<F: Float>(t1458: F, t2039: F, t27863: F, t32350: F, t33152: F, t33154: F, t33583: F, t33585: F, t33587: F, t33595: F, t33598: F, t33600: F, t33690: F, t34137: F, t7266: F, t7801: F, t8446: F) -> (F,) {
    let t34146 = 2.0 * t1458 * t32350 + 2.0 * t2039 * t27863 + 2.0 * t2039 * t33690 + 2.0 * t7266 * t7801 + t33152 + t33154 + t33583 + t33585 + t33587 + t33595 + t33598 + t33600 + t34137 + t8446;
    (t34146,)
}
