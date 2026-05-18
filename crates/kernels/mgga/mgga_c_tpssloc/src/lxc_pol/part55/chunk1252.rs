//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1252/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1252<F: Float>(t3701: F, t5187: F, t1983: F, t31084: F, t33157: F, t6876: F, t31036: F, t7685: F, t40611: F, t8492: F, t26161: F, t26163: F) -> (F, F, F, F) {
    let t120669 = t3701 * t5187;
    let t120672 = F::new(3.0) * t1983 * t31084 * t120669;
    let t120677 = t6876 * t33157;
    let t120683 = F::new(2.0) * t7685 * t31036;
    let t120684 = t8492 * t40611;
    let t120687 = F::new(6.0) * t26161 * t120684 * t26163;
    (t120672, t120677, t120683, t120687)
}
