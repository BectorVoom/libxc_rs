//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 820/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk820<F: Float>(t109: F, t2180: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t8143: F, t8153: F, t8161: F, t1774: F, t1453: F, t8129: F, t1444: F, t8138: F, t8127: F, t8128: F, t8137: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t8166 = t2180 * t671;
    let t8171 = 0.45e1 * t8153 * t577 + 0.135e2 * t8161 * t671 + 0.135e2 * t3938 * t2180 + 27.0 * t3941 * t8166 + 0.135e2 * t1401 * t8143;
    let t8221 = t1774 * t2180;
    let t8223 = t8129 * t1453;
    let t8226 = t8138 * t1444;
    let t8230 = piecewise3(t110, 0.0, t8127 + t8128 * t8223 / 4.0 - 5.0 / 24.0 * t8137 * t8226);
    (t8166, t8171, t8221, t8223, t8226, t8230)
}
