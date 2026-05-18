//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 898/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk898<F: Float>(t1268: F, t2181: F, t2183: F, t2314: F, t4034: F, t5113: F, t652: F, t8124: F, t8144: F, t8148: F, t8150: F) -> F {
    let t8153 = F::new(2.0) * t1268 * t8148 + F::new(2.0) * t1268 * t8150 - F::new(2.0) * t2181 * t2314 - F::new(2.0) * t2181 * t4034 + F::new(2.0) * t2183 * t2314 + F::new(2.0) * t2183 * t5113 - F::new(2.0) * t652 * t8124 - F::new(2.0) * t652 * t8144;
    t8153
}
