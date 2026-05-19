//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 887/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk887<F: Float>(t1375: F, t2016: F, t2092: F, t568: F, t6958: F, t7194: F, t8457: F, t8461: F, t8613: F, t8618: F, t8623: F, t8627: F, t8637: F) -> F {
    let t8639 = t8457 - t8461 + F::cast_from(0.82246703342411321825e-2_f64) * t8613 + t8618 * t568 - t7194 * t2016 - F::cast_from(0.82246703342411321825e-2_f64) * t8623 - t6958 * t2092 + F::new(2.0) * t1375 * t8627 - t1375 * t8637;
    t8639
}
