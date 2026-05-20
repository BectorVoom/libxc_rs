//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1228/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1228<F: Float>(t33398: F, t858: F, t26728: F, t7516: F, t6571: F, t7841: F, t6553: F, t1880: F, t1492: F, t8543: F, t218: F, t33395: F) -> (F, F, F, F, F, F, F) {
    let t33399 = t858 * t33398;
    let t33405 = t26728 * t7516;
    let t33408 = t6571 * t7841;
    let t33409 = t6553 * t33408;
    let t33410 = t1880 * t33409;
    let t33412 = t1492 * t8543;
    let t33414 = t218 * t33395;
    (t33399, t33405, t33408, t33409, t33410, t33412, t33414)
}
