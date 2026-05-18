//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1027/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1027<F: Float>(t28025: F, t7042: F, t28827: F, t8607: F, t33336: F, t7685: F, t28821: F, t8644: F, t1799: F, t22574: F, t26558: F, t33221: F) -> (F, F, F, F, F) {
    let t128543 = F::new(2.0) * t7042 * t28025;
    let t128549 = F::new(6.0) * t8607 * t28827;
    let t128551 = F::new(2.0) * t7685 * t33336;
    let t128552 = t28821 * t8644;
    let t128562 = F::new(12.0) * t22574 * t26558 * t33221 * t1799;
    (t128543, t128549, t128551, t128552, t128562)
}
