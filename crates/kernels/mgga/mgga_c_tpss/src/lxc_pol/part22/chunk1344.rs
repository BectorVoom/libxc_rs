//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1344/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1344<F: Float>(t63907: F, t63913: F, t63917: F, t63899: F, t63901: F, t63903: F, t63905: F, t63909: F, t63911: F, t63921: F, t63923: F, t63925: F) -> F {
    let t66390 = F::new(7.0) / F::new(144.0) * t63907;
    let t66393 = F::new(7.0) / F::new(144.0) * t63913;
    let t66394 = F::new(7.0) / F::new(288.0) * t63917;
    let t66398 = -t63899 / F::new(384.0) - t63901 / F::new(768.0) + t63903 / F::new(96.0) + t63905 / F::new(192.0) - t66390 + t63909 / F::new(192.0) + t63911 / F::new(96.0) - t66393 - t66394 - t63921 / F::new(128.0) + t63923 / F::new(128.0) - t63925 / F::new(768.0);
    t66398
}
