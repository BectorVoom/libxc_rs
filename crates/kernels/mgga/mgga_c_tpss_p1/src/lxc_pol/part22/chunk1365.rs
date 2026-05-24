//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1365/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1365<F: Float>(t65570: F, t65592: F, t65600: F, t65572: F, t65574: F, t65576: F, t65578: F, t65580: F, t65582: F, t65584: F, t65586: F, t65588: F, t65597: F) -> F {
    let t67150 = F::new(7.0) / F::new(36.0) * t65570;
    let t67160 = F::new(7.0) / F::new(288.0) * t65592;
    let t67162 = F::new(7.0) / F::new(12.0) * t65600;
    let t67163 = t67150 - t65572 / F::new(24.0) + t65574 / F::new(192.0) - t65576 / F::new(384.0) - t65578 / F::new(768.0) + t65580 / F::new(128.0) + t65582 / F::new(96.0) + t65584 / F::new(192.0) - t65586 / F::new(96.0) - F::new(5.0) / F::new(192.0) * t65588 - t67160 - t65597 / F::new(2.0) - t67162;
    t67163
}
