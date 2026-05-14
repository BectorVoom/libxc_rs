//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1165/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1165<F: Float>(t30071: F, t510: F, t2199: F, t3652: F, t574: F, t1393: F, t8189: F, t1268: F, t12734: F, t12739: F, t12823: F, t2200: F, t2202: F, t2314: F, t30035: F, t30038: F, t4034: F, t5113: F, t652: F, t8176: F, t8190: F, t8194: F, t8196: F, t9348: F) -> (F, F, F, F, F) {
    let t30072 = t510 * t30071;
    let t30085 = t3652 * t2199;
    let t30088 = t30071 * t574;
    let t30091 = t8189 * t1393;
    let t30094 = 2.0 * t1268 * t30035 + 2.0 * t1268 * t30088 + 4.0 * t1268 * t30091 - 4.0 * t12734 * t2200 + 4.0 * t12734 * t2202 + 2.0 * t12739 * t2202 - 2.0 * t12823 * t2200 - 2.0 * t2200 * t9348 + 2.0 * t2202 * t9348 - 4.0 * t2314 * t8176 - 4.0 * t2314 * t8190 + 4.0 * t2314 * t8194 + 4.0 * t2314 * t8196 - 4.0 * t30038 * t652 - 2.0 * t30072 * t652 - 2.0 * t30085 * t652 - 4.0 * t4034 * t8176 - 4.0 * t4034 * t8190 + 4.0 * t5113 * t8194 + 4.0 * t5113 * t8196;
    (t30072, t30085, t30088, t30091, t30094)
}
