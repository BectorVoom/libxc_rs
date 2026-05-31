//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1241/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1241<F: Float>(t12461: F, t8488: F, t26161: F, t26163: F, t114360: F, t25971: F, t33129: F, t6876: F, t32670: F, t4034: F, t1266: F, t33094: F) -> (F, F, F, F, F) {
    let t120100 = t8488 * t12461;
    let t120103 = F::cast_from(2.0_f64) * t26161 * t120100 * t26163;
    let t120104 = t114360 * t25971;
    let t120107 = F::cast_from(3.0_f64) * t6876 * t33129;
    let t120108 = t4034 * t32670;
    let t120111 = F::cast_from(2.0_f64) * t33094 * t1266;
    (t120103, t120104, t120107, t120108, t120111)
}
