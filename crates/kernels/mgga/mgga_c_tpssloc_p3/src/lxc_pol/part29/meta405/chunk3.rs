//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1654/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1654<F: Float>(t11888: F, t11904: F, t11907: F, t11914: F, t1201: F, t1244: F, t1247: F, t15032: F, t15241: F, t15245: F, t15248: F, t15253: F, t15257: F, t15426: F, t15430: F, t15772: F, t15777: F, t1758: F, t3565: F, t3604: F, t3610: F, t3621: F, t3624: F, t3626: F, t470: F, t494: F, t5064: F, t5069: F, t5076: F, t5080: F, t5084: F, t5086: F) -> F {
    let t15785 = F::cast_from(2.0_f64) * t15032 * t1247 + t1244 * t15241 - F::cast_from(2.0_f64) * t11907 * t5080 - t15245 * t3626 - F::cast_from(6.0_f64) * t11888 * t15248 + F::cast_from(2.0_f64) * t3604 * t5076 + F::cast_from(2.0_f64) * t3610 * t15253 + t3565 * t1758 - F::cast_from(2.0_f64) * t3624 * t15257 + t15426 * t494 + t11914 * t15430 + t470 * t15772 + F::cast_from(2.0_f64) * t1201 * t5086 + F::cast_from(2.0_f64) * t1244 * t15777 + t5064 * t3621 + F::cast_from(2.0_f64) * t3604 * t5084 + F::cast_from(4.0_f64) * t11904 * t5069;
    t15785
}
