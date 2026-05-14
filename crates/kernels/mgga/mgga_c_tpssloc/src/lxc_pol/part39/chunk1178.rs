//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1178/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1178<F: Float>(t2205: F, t2319: F, t1268: F, t12725: F, t12734: F, t12739: F, t12823: F, t1849: F, t19456: F, t2200: F, t2202: F, t26114: F, t26117: F, t30035: F, t30071: F, t30266: F, t30269: F, t30272: F, t30330: F, t4028: F, t4034: F, t5113: F, t55934: F, t55962: F, t8176: F, t8190: F, t8194: F, t8274: F, t8278: F, t8280: F, t90370: F, t90375: F, t9348: F) -> (F, F) {
    let t110926 = t2205 * t2319;
    let t110972 = 2.0 * t1268 * t1849 * t30071 - 4.0 * t12725 * t8176 + 4.0 * t12725 * t8194 - 4.0 * t12734 * t8274 + 2.0 * t12739 * t8280 - 2.0 * t12823 * t8274 - 4.0 * t19456 * t8190 + 4.0 * t19456 * t8194 - 4.0 * t2200 * t90370 + 4.0 * t2202 * t55934 + 2.0 * t2202 * t55962 + 2.0 * t2202 * t90375 - 4.0 * t26114 * t8176 + 4.0 * t26117 * t8194 + 2.0 * t30035 * t4028 + 4.0 * t30266 * t5113 + 4.0 * t30269 * t5113 - 4.0 * t30272 * t4034 + 4.0 * t30330 * t5113 + 2.0 * t8278 * t9348;
    (t110926, t110972)
}
