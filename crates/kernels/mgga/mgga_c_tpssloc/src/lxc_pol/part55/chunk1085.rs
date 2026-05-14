//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1085/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1085<F: Float>(t120100: F, t26161: F, t26163: F, t33129: F, t6876: F, t1266: F, t33094: F, t4025: F, t8319: F, t510: F, t19456: F, t8326: F, t26114: F, t26117: F, t31717: F, t7467: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120103 = 2.0 * t26161 * t120100 * t26163;
    let t120107 = 3.0 * t6876 * t33129;
    let t120111 = 2.0 * t33094 * t1266;
    let t120112 = t4025 * t8319;
    let t120114 = 2.0 * t120112 * t510;
    let t120120 = t19456 * t8326;
    let t120121 = 2.0 * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = 2.0 * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = 2.0 * t120124;
    let t120127 = 4.0 * t31717 * t7467;
    (t120103, t120107, t120111, t120112, t120114, t120121, t120123, t120125, t120127)
}
