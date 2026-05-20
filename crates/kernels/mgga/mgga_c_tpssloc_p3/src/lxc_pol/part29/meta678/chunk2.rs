//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2272/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272<F: Float>(t2113: F, t2363: F, t12557: F, t1459: F, t1774: F, t24543: F, t24545: F, t24932: F, t27888: F, t4028: F, t4037: F, t4073: F, t652: F, t7266: F, t8103: F, t85428: F, t90421: F, t90428: F, t90434: F, t90436: F, t90440: F, t90444: F, t90447: F, t90450: F, t90454: F, t90456: F) -> (F, F) {
    let t94248 = t2113 * t2363;
    let t94257 = -F::new(2.0) * t2363 * t652 * t8103 - F::new(2.0) * t12557 * t7266 - F::new(2.0) * t1459 * t85428 - F::new(2.0) * t1459 * t94248 - t1774 * t24543 - F::new(4.0) * t24545 * t4028 - F::new(4.0) * t24932 * t4073 - F::new(4.0) * t27888 * t4037 - F::new(4.0) * t27888 * t4073 + t90421 - t90428 + t90434 - t90436 + t90440 + t90444 + t90447 - t90450 - t90454 - t90456;
    (t94248, t94257)
}
