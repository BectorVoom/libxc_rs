//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1346/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1346<F: Float>(t63957: F, t63960: F, t63964: F, t63966: F, t61063: F, t61065: F, t61073: F, t62711: F, t63951: F, t63953: F, t63955: F, t63962: F, t63968: F) -> F {
    let t66418 = F::new(35.0) / F::new(108.0) * t63957;
    let t66420 = F::new(7.0) / F::new(144.0) * t63960;
    let t66422 = F::new(119.0) / F::new(864.0) * t63964;
    let t66423 = F::new(7.0) / F::new(36.0) * t63966;
    let t66425 = -F::new(35.0) / F::new(54.0) * t61063 + F::new(7.0) / F::new(72.0) * t61065 - t63951 / F::new(48.0) + t63953 / F::new(192.0) + t63955 / F::new(384.0) - t66418 - F::new(7.0) / F::new(24.0) * t61073 + t66420 - t63962 / F::new(192.0) - t66422 - t62711 + t66423 - t63968 / F::new(24.0);
    t66425
}
