//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 905/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk905<F: Float>(t1268: F, t2181: F, t2183: F, t4028: F, t652: F, t7458: F, t7676: F, t8221: F, t8231: F, t8235: F, t8237: F) -> F {
    let t8240 = F::new(2.0) * t1268 * t8235 + F::new(2.0) * t1268 * t8237 - F::new(2.0) * t2181 * t4028 - F::new(2.0) * t2181 * t7458 + F::new(2.0) * t2183 * t4028 + F::new(2.0) * t2183 * t7676 - F::new(2.0) * t652 * t8221 - F::new(2.0) * t652 * t8231;
    t8240
}
