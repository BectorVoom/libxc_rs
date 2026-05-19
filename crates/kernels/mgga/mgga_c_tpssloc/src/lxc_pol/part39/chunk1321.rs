//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1321/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1321<F: Float>(t109: F, t111096: F, t111141: F, t1268: F, t12734: F, t19456: F, t2200: F, t2202: F, t2314: F, t26114: F, t30035: F, t30072: F, t30088: F, t30091: F, t30266: F, t30272: F, t30316: F, t30321: F, t30326: F, t4028: F, t4034: F, t5107: F, t5361: F, t55934: F, t574: F, t652: F, t7676: F, t8189: F, t8190: F, t8194: F, t8196: F, t8260: F, t8280: F, t90370: F, t90381: F, t9348: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t111143 = piecewise3::<F>(t110, F::new(0.0), t111096 + t111141);
    let t111168 = F::new(4.0) * t1268 * t8189 * t5361 - F::new(4.0) * t4034 * t30316 - F::new(4.0) * t2314 * t30272 - F::new(4.0) * t12734 * t8260 - F::new(4.0) * t2314 * t30326 + F::new(2.0) * t9348 * t8280 + F::new(4.0) * t4028 * t30091 - F::new(2.0) * t4028 * t30072 - F::new(2.0) * t90381 * t2200 + F::new(2.0) * t1268 * t111143 * t574 + F::new(4.0) * t90370 * t2202 + F::new(4.0) * t26114 * t8194 - F::new(4.0) * t55934 * t2200 + F::new(4.0) * t2314 * t30321 + F::new(2.0) * t7676 * t30035 + F::new(2.0) * t7676 * t30088 - F::new(4.0) * t652 * t5107 * t8189 + F::new(4.0) * t2314 * t30266 + F::new(4.0) * t19456 * t8196 - F::new(4.0) * t26114 * t8190;
    (t111143, t111168)
}
