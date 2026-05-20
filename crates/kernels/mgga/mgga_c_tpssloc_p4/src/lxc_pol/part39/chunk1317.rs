//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1317/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1317<F: Float>(t1268: F, t12725: F, t12739: F, t12823: F, t1393: F, t16503: F, t1774: F, t19456: F, t2199: F, t2200: F, t2314: F, t26179: F, t30038: F, t30071: F, t30072: F, t30085: F, t30088: F, t30091: F, t30274: F, t30315: F, t30326: F, t3929: F, t4028: F, t4034: F, t45632: F, t55962: F, t652: F, t7458: F, t7676: F, t8176: F, t8190: F, t8196: F, t8260: F, t8273: F, t8274: F, t8278: F, t9348: F) -> F {
    let t111017 = F::new(4.0) * t7676 * t30091 - F::new(2.0) * t652 * t1774 * t30071 - F::new(2.0) * t9348 * t8274 + F::new(2.0) * t4028 * t30088 + F::new(2.0) * t1268 * t8273 * t3929 + F::new(4.0) * t1268 * t30315 * t1393 - F::new(2.0) * t7458 * t30072 - F::new(2.0) * t9348 * t8260 - F::new(4.0) * t7458 * t30038 - F::new(2.0) * t45632 * t2200 - F::new(2.0) * t55962 * t2200 - F::new(4.0) * t19456 * t8176 - F::new(4.0) * t2314 * t30274 - F::new(2.0) * t12823 * t8260 - F::new(4.0) * t4034 * t30326 + F::new(2.0) * t1268 * t2199 * t16503 + F::new(2.0) * t12739 * t8278 + F::new(4.0) * t12725 * t8196 - F::new(4.0) * t26179 * t8190 - F::new(2.0) * t7458 * t30085;
    t111017
}
