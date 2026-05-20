//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2766/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766<F: Float>(t13396: F, t1499: F, t13380: F, t13398: F, t13414: F, t13423: F, t13448: F, t16673: F, t16679: F, t16935: F, t2617: F, t2729: F, t2733: F, t2736: F, t40895: F, t4166: F, t4182: F, t4234: F, t4281: F, t4291: F, t5585: F, t5645: F, t58204: F, t812: F, t9612: F) -> F {
    let t58313 = t1499 * t13396;
    let t58337 = F::new(8.0) * t13380 * t16935 * t4281 - F::new(4.0) * t13380 * t4234 * t4291 + F::new(2.0) * t40895 * t5585 * t812 + F::new(8.0) * t4182 * t4281 * t58204 - F::new(12.0) * t13398 * t58313 - F::new(2.0) * t13414 * t4166 - F::new(2.0) * t13423 * t4166 + F::new(2.0) * t13448 * t1499 + F::new(2.0) * t16673 * t2729 - F::new(2.0) * t16673 * t2733 - t16673 * t2736 - F::new(4.0) * t16679 * t2617 + F::new(2.0) * t5645 * t9612;
    t58337
}
