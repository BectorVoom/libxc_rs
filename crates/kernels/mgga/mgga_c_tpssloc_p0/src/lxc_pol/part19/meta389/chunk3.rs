//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1465/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1465<F: Float>(t11638: F, t486: F, t11818: F, t1213: F, t248: F, t3494: F, t3506: F, t3509: F, t3515: F, t3516: F, t11718: F, t44857: F) -> (F, F, F, F, F) {
    let t44879 = t486 * t11638;
    let t44886 = t1213 * t248 * t11818 * t3494;
    let t44890 = t3506 * t248 * t11818 * t3509;
    let t44894 = t3515 * t248 * t11818 * t3516;
    let t44896 = t44857 * t11718;
    (t44879, t44886, t44890, t44894, t44896)
}
