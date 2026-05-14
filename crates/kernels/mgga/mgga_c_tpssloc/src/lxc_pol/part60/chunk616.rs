//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 616/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk616<F: Float>(t1375: F, t2016: F, t2092: F, t568: F, t6958: F, t7194: F, t8457: F, t8461: F, t8613: F, t8618: F, t8623: F, t8627: F, t8637: F, t533: F, t1390: F, t1983: F) -> (F, F, F, F) {
    let t8639 = t8457 - t8461 + 0.82246703342411321825e-2 * t8613 + t8618 * t568 - t7194 * t2016 - 0.82246703342411321825e-2 * t8623 - t6958 * t2092 + 2.0 * t1375 * t8627 - t1375 * t8637;
    let t8640 = t533 * t8639;
    let t8641 = t8640 * t1390;
    let t8642 = t1983 * t8641;
    (t8639, t8640, t8641, t8642)
}
