//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1356/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1356<F: Float>(t20882: F, t23146: F, t20988: F, t25084: F, t20891: F, t1898: F, t20937: F, t249: F, t20983: F, t105278: F, t105282: F, t105286: F, t105288: F, t105290: F, t81736: F, t81743: F, t87213: F, t87243: F, t98618: F, t98647: F, t98690: F, t98694: F, t98696: F) -> F {
    let t105292 = t23146 * t20882;
    let t105294 = t25084 * t20988;
    let t105296 = t23146 * t20891;
    let t105299 = t20937 * t1898 * t249;
    let t105304 = t25084 * t20983;
    let t105308 = -F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t98618 + F::cast_from(0.60559134141210586281e-3_f64) * t98647 - t81736 + F::cast_from(0.36335480484726351768e-2_f64) * t105278 + F::cast_from(0.36335480484726351768e-2_f64) * t105282 - F::cast_from(0.72670960969452703536e-2_f64) * t105286 + t105288 / F::cast_from(128.0_f64) + t105290 / F::cast_from(64.0_f64) + t105292 / F::cast_from(128.0_f64) + t105294 / F::cast_from(256.0_f64) - t105296 / F::cast_from(512.0_f64) + t81743 + t105299 / F::cast_from(1536.0_f64) + F::cast_from(0.50465945117675488567e-4_f64) * t87213 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t98690 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t87243 - t105304 / F::cast_from(64.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t98694 + F::cast_from(0.25434836339308446238e-1_f64) * t98696;
    t105308
}
