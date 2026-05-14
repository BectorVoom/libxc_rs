//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1026/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1026<F: Float>(t14457: F, t2515: F, t141: F, t3431: F, t3748: F, t4573: F, t8493: F, t581: F, t8633: F, t4826: F, t861: F, t3753: F, t2464: F, t4579: F, t2459: F, t14452: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14461 = t2515 * t14457;
    let t14462 = t141 * t14461;
    let t14464 = t3748 * t3431;
    let t14465 = t2515 * t14464;
    let t14466 = t141 * t14465;
    let t14468 = t8493 * t4573;
    let t14469 = t14468 * t581;
    let t14470 = t8633 * t14469;
    let t14471 = t141 * t14470;
    let t14473 = t4826 * t581;
    let t14474 = t861 * t14473;
    let t14475 = t141 * t14474;
    let t14477 = t3753 * t3431;
    let t14478 = t861 * t14477;
    let t14479 = t141 * t14478;
    let t14481 = t2464 * t4579;
    let t14482 = t14481 * t581;
    let t14483 = t861 * t14482;
    let t14484 = t141 * t14483;
    let t14486 = t2459 * t4579;
    let t14487 = t14486 * t581;
    let t14488 = t2515 * t14487;
    let t14489 = t141 * t14488;
    let t14491 = t835 * t14452;
    (t14462, t14464, t14466, t14469, t14471, t14473, t14475, t14477, t14479, t14482, t14484, t14487, t14489, t14491)
}
