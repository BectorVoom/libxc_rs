//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1060/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1060<F: Float>(t127539: F, t128551: F, t128552: F, t128562: F, t128564: F, t128567: F, t128571: F, t128573: F, t128575: F, t128577: F, t128581: F, t128584: F, t128588: F, t128592: F, t2165: F, t28943: F, t28969: F, t33746: F, t7904: F, t7943: F, t8690: F) -> F {
    let t130472 = -t2165 * t28943 + F::cast_from(3.0_f64) * t28969 * t8690 + F::cast_from(6.0_f64) * t33746 * t7904 - F::cast_from(2.0_f64) * t33746 * t7943 - t127539 + t128551 - t128552 + t128562 + t128564 + t128567 + t128571 - t128573 - t128575 - t128577 - t128581 + t128584 + t128588 - t128592;
    t130472
}
