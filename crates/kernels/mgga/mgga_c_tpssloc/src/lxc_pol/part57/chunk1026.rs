//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1026/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1026<F: Float>(t1992: F, t33249: F, t90566: F, t115447: F, t120383: F, t124139: F, t124142: F, t124146: F, t127252: F, t127254: F, t127256: F, t127258: F, t127263: F, t127267: F, t127270: F, t127273: F) -> (F, F) {
    let t128604 = t1992 * t90566 * t33249;
    let t128616 = -t127252 / F::new(192.0) - t127254 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t127256 + t124139 + t127258 / F::new(96.0) + t124142 - F::new(0.96894614625936938046e-2) * t127263 + F::new(0.67826230238155856632e-1) * t127267 + F::new(0.19378922925187387609e-1) * t127270 - t115447 - F::new(0.16149102437656156341e-2) * t127273 + t124146 + F::new(0.13565246047631171327e0) * t120383;
    (t128604, t128616)
}
