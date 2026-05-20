//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2185/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185<F: Float>(t11719: F, t11722: F, t248: F, t3570: F, t11818: F, t1213: F, t3494: F, t3506: F, t3509: F, t3515: F, t3516: F, t11718: F, t44857: F) -> (F, F, F, F, F) {
    let t44871 = t11719 * t248 * t3570 * t11722;
    let t44886 = t1213 * t248 * t11818 * t3494;
    let t44890 = t3506 * t248 * t11818 * t3509;
    let t44894 = t3515 * t248 * t11818 * t3516;
    let t44896 = t44857 * t11718;
    (t44871, t44886, t44890, t44894, t44896)
}
