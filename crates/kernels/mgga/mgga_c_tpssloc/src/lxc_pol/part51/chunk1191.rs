//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1191/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1191<F: Float>(t2039: F, t22461: F, t26103: F, t31237: F, t31239: F, t31532: F, t31700: F, t31704: F, t31706: F, t31708: F, t31716: F, t31719: F, t31721: F, t6517: F, t671: F, t7056: F, t8446: F) -> F {
    let t31722 = F::new(2.0) * t2039 * t22461 + F::new(2.0) * t2039 * t26103 + F::new(2.0) * t31532 * t671 + F::new(2.0) * t6517 * t7056 + t31237 + t31239 + t31700 + t31704 + t31706 + t31708 + t31716 + t31719 + t31721 + t8446;
    t31722
}
