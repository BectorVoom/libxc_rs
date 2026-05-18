//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1022/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1022<F: Float>(t23384: F, t7604: F, t1615: F, t6768: F, t1060: F, t2987: F, t4343: F, t4338: F, t4509: F, t4640: F, t6754: F, t1611: F, t6764: F) -> (F, F, F, F, F, F) {
    let t25563 = t23384 * t7604;
    let t25567 = t6768 * t1615;
    let t25568 = t25567 * t1060;
    let t25571 = t2987 * t4343;
    let t25574 = t4509 * t4338;
    let t25577 = t4640 * t6754;
    let t25580 = t1611 * t6764;
    (t25563, t25568, t25571, t25574, t25577, t25580)
}
