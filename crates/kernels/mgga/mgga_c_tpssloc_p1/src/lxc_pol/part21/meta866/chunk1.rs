//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3161/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161<F: Float>(t3590: F, t6224: F, t11877: F, t11904: F, t11907: F, t1244: F, t1246: F, t14989: F, t15004: F, t15027: F, t15032: F, t15248: F, t19123: F, t19139: F, t19189: F, t19201: F, t19204: F, t3617: F, t3624: F, t3625: F, t5011: F, t5052: F, t5064: F, t5079: F, t5080: F, t5084: F, t52435: F, t53565: F, t6261: F) -> (F, F) {
    let t65347 = t3590 * t6224;
    let t65374 = F::cast_from(4.0_f64) * t1244 * t1246 * t5011 * t5052 - F::cast_from(2.0_f64) * t19189 * t3624 * t5079 - t3624 * t3625 * t65347 + t11877 * t6261 + F::cast_from(4.0_f64) * t11904 * t19123 + F::cast_from(8.0_f64) * t11904 * t19204 - F::cast_from(4.0_f64) * t11907 * t19139 + F::cast_from(4.0_f64) * t14989 * t5064 + F::cast_from(8.0_f64) * t15004 * t15027 + F::cast_from(4.0_f64) * t15032 * t5084 - F::cast_from(12.0_f64) * t15248 * t53565 + F::cast_from(2.0_f64) * t19201 * t3617 - F::cast_from(4.0_f64) * t5080 * t52435;
    (t65347, t65374)
}
