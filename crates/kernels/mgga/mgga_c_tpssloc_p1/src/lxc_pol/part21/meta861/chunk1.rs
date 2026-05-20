//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3123/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123<F: Float>(t18287: F, t225: F, t11925: F, t11928: F, t1235: F, t1252: F, t14980: F, t15771: F, t15789: F, t15790: F, t15797: F, t15803: F, t1720: F, t1761: F, t18571: F, t19209: F, t19249: F, t27784: F, t3590: F, t3593: F, t3600: F, t4945: F, t498: F, t5055: F, t5089: F, t53677: F, t53703: F, t6150: F, t6244: F, t6268: F) -> F {
    let t64595 = t18287 * t225;
    let t64602 = F::new(2.0) * t1235 * t18571 * t498 + F::new(2.0) * t15771 * t1720 * t498 - F::new(24.0) * t15789 * t27784 * t53677 + t3590 * t498 * t6150 + F::new(2.0) * t11925 * t6244 + F::new(2.0) * t11928 * t6244 - t11928 * t6268 - F::new(2.0) * t1252 * t64595 - F::new(4.0) * t14980 * t5089 + F::new(8.0) * t15790 * t4945 + F::new(8.0) * t15790 * t5055 - F::new(4.0) * t15797 * t5089 + F::new(4.0) * t15803 * t4945 - F::new(4.0) * t1761 * t53703 - F::new(2.0) * t19209 * t3593 + F::new(2.0) * t19249 * t3600;
    t64602
}
