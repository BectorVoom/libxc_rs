//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1175/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1175<F: Float>(t2096: F, t3524: F, t108: F, t555: F, t22: F, t3528: F, t105: F, t13178: F, t13181: F, t13182: F, t13185: F, t13188: F, t13191: F, t13199: F, t13202: F, t13203: F, t1325: F, t1327: F, t2078: F, t2093: F, t2097: F, t3515: F, t3519: F, t631: F, t97: F) -> F {
    let t13206 = t3524 * t2096;
    let t13209 = t108 * t555;
    let t13212 = t3528 * t22;
    let t13215 = F::new(200.0) / F::new(27.0) * t2078 * t1325 - F::new(100.0) / F::new(27.0) * t631 * t3515 - F::new(50.0) / F::new(9.0) * t631 * t3519 - F::new(10.0) / F::new(27.0) * t97 * t13178 + F::new(20.0) / F::new(9.0) * t13181 * t13182 + F::new(10.0) / F::new(9.0) * t97 * t13185 + F::new(5.0) / F::new(3.0) * t97 * t13188 - F::new(5.0) * t97 * t13191 - F::new(50.0) / F::new(27.0) * t1327 * t2093 - F::new(25.0) / F::new(9.0) * t1327 * t2097 - F::new(10.0) / F::new(27.0) * t105 * t13199 - F::new(20.0) / F::new(9.0) * t13202 * t13203 + F::new(10.0) / F::new(9.0) * t105 * t13206 - F::new(5.0) / F::new(3.0) * t105 * t13209 + F::new(5.0) * t105 * t13212;
    t13215
}
