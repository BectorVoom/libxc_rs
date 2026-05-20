//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1623/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1623<F: Float>(t1238: F, t15820: F, t1761: F, t18287: F, t19121: F, t19209: F, t19211: F, t19214: F, t19220: F, t19226: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5060: F, t6268: F) -> F {
    let t19231 = -t1238 * t19209 + F::new(4.0) * t1238 * t19214 + F::new(2.0) * t1238 * t19220 - F::new(6.0) * t1238 * t19226 - F::new(2.0) * t15820 * t1761 + t18287 * t498 + t19121 * t498 + t19211 * t498 - t3487 * t6268 - t3593 * t6268 + F::new(4.0) * t4945 * t5060 + F::new(4.0) * t5055 * t5060;
    t19231
}
