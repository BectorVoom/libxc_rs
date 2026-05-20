//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1242/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1242<F: Float>(t1458: F, t2039: F, t24999: F, t31532: F, t33085: F, t33152: F, t33154: F, t33579: F, t33583: F, t33585: F, t33587: F, t33595: F, t33598: F, t33600: F, t6517: F, t7801: F, t8446: F) -> F {
    let t33601 = F::new(2.0) * t1458 * t31532 + F::new(2.0) * t2039 * t24999 + F::new(2.0) * t2039 * t33085 + F::new(2.0) * t6517 * t7801 + t33152 + t33154 + t33579 + t33583 + t33585 + t33587 + t33595 + t33598 + t33600 + t8446;
    t33601
}
