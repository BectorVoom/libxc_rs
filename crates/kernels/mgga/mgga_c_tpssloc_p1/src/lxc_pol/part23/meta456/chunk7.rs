//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1326/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1326<F: Float>(t13228: F, t13416: F, t1525: F, t16673: F, t20853: F, t20854: F, t20861: F, t20870: F, t20871: F, t20876: F, t20937: F, t2728: F, t4166: F, t4281: F, t4295: F, t5575: F, t5645: F, t5655: F, t67392: F, t76290: F, t812: F) -> F {
    let t76467 = F::new(8.0) * t13228 * t4281 * t67392 + F::new(24.0) * t13416 * t20861 * t812 - F::new(4.0) * t20853 * t4295 * t812 - F::new(4.0) * t20870 * t4295 * t812 + F::new(14.0) * t2728 * t76290 * t812 + F::new(4.0) * t1525 * t20937 + F::new(12.0) * t16673 * t5645 - F::new(4.0) * t20854 * t4166 - F::new(4.0) * t20871 * t4166 - F::new(12.0) * t20876 * t4166 + F::new(6.0) * t5575 * t5655;
    t76467
}
