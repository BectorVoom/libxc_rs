//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 888/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk888<F: Float>(t1268: F, t2200: F, t2202: F, t4028: F, t652: F, t7458: F, t7676: F, t8260: F, t8274: F, t8278: F, t8280: F) -> F {
    let t8283 = F::new(2.0) * t1268 * t8278 + F::new(2.0) * t1268 * t8280 - F::new(2.0) * t2200 * t4028 - F::new(2.0) * t2200 * t7458 + F::new(2.0) * t2202 * t4028 + F::new(2.0) * t2202 * t7676 - F::new(2.0) * t652 * t8260 - F::new(2.0) * t652 * t8274;
    t8283
}
