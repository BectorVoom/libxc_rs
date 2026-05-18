//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1298/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1298<F: Float>(t112: F, t30581: F, t111808: F, t1268: F, t1849: F, t19451: F, t19456: F, t2200: F, t2202: F, t2314: F, t26117: F, t28007: F, t30266: F, t30315: F, t30538: F, t30543: F, t30565: F, t4028: F, t4034: F, t5107: F, t5113: F, t574: F, t652: F, t75560: F, t7676: F, t8176: F, t8190: F, t8194: F, t8196: F, t8273: F, t8280: F, t96657: F, t96709: F) -> (F, F) {
    let t111845 = t30581 * t112;
    let t111916 = F::new(2.0) * t111808 * t1268 * t574 + F::new(4.0) * t1268 * t1849 * t30315 - F::new(4.0) * t5107 * t652 * t8273 - F::new(2.0) * t19451 * t8176 - F::new(2.0) * t19451 * t8190 + F::new(4.0) * t19456 * t8280 - F::new(2.0) * t2200 * t75560 + F::new(2.0) * t2202 * t75560 + F::new(2.0) * t2202 * t96657 + F::new(2.0) * t2202 * t96709 + F::new(4.0) * t2314 * t30538 - F::new(4.0) * t2314 * t30543 + F::new(2.0) * t2314 * t30565 + F::new(4.0) * t26117 * t8280 + F::new(2.0) * t28007 * t8194 + F::new(2.0) * t28007 * t8196 + F::new(4.0) * t30266 * t4028 + F::new(4.0) * t30266 * t7676 + F::new(4.0) * t30538 * t5113 - F::new(4.0) * t30543 * t4034;
    (t111845, t111916)
}
