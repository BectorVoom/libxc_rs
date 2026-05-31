//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 173/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk173<F: Float>(t576: F, t577: F, t11: F, t2: F, t10: F, t3: F, t9: F, t16: F, t15: F, t14: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t580 = F::cast_from(1.0_f64) + F::cast_from(0.45e1_f64) * t576 * t577;
    let t581 = t2 * t11;
    let t582 = F::cast_from(0.174e1_f64) * t581;
    let t583 = t10 * t3;
    let t584 = F::cast_from(1.0_f64) / t583;
    let t586 = F::cast_from(0.174e1_f64) * t9 * t584;
    let t587 = t9 * t2;
    let t588 = t587 * t16;
    let t589 = F::cast_from(2.0_f64) * t588;
    let t590 = t15 * t3;
    let t591 = F::cast_from(1.0_f64) / t590;
    let t592 = t14 * t591;
    (t580, t581, t582, t583, t584, t586, t587, t588, t589, t590, t591, t592)
}
