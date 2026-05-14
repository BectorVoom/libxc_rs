//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 559/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk559<F: Float>(t5456: F, t89: F, t1458: F, t1774: F, t1453: F, t2331: F, t1444: F, t2341: F, t5396: F, t95: F, t1419: F, t1449: F, t2349: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t92: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5457 = t89 * t5456;
    let t5460 = t1774 * t1458;
    let t5464 = t1453 * t1453;
    let t5465 = t2331 * t5464;
    let t5468 = t1444 * t1444;
    let t5469 = t2341 * t5468;
    let t5472 = t95 * t5396;
    let t5475 = tau1 * t1419;
    let t5480 = t1449 * t1449;
    let t5481 = t2349 * t5480;
    let t5484 = -t5396;
    let t5485 = t103 * t5484;
    let t5488 = 10.0 / 9.0 * t92 * t5469 + 5.0 / 3.0 * t92 * t5472 + 40.0 / 9.0 * t5475 * t104 - 50.0 / 9.0 * t1447 * t1450 + 10.0 / 9.0 * t100 * t5481 + 5.0 / 3.0 * t100 * t5485;
    (t5457, t5460, t5464, t5465, t5468, t5475, t5480, t5481, t5484, t5485, t5488)
}
