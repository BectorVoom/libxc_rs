//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2231/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2231<F: Float>(t17046: F, t1888: F, t6646: F, t1510: F, t22986: F, t87130: F, t25249: F, t4234: F, t23110: F, t28337: F, t81651: F, t13176: F, t1499: F, t22992: F, t25295: F, t5617: F, t7533: F, t812: F, t81595: F, t81599: F, t81600: F, t81602: F, t92513: F, t98416: F, t98420: F, t98425: F, t98428: F, t98432: F) -> F {
    let t98435 = t1888 * t6646 * t17046;
    let t98439 = t22986 * t6646 * t87130 * t1510;
    let t98443 = t22986 * t6646 * t25249 * t4234;
    let t98446 = t81651 * t23110 * t28337;
    let t98450 = -F::cast_from(0.82246703342411321824e-2_f64) * t81595 + F::new(2.0) * t1499 * t25295 - t81599 + F::cast_from(0.26044789391763585244e-1_f64) * t81600 + F::cast_from(0.63969658155208805863e-1_f64) * t81602 + F::cast_from(0.76763589786250567037e-1_f64) * t98416 + t92513 - F::new(2.0) * t13176 * t7533 - F::cast_from(0.76763589786250567037e-1_f64) * t98420 + F::cast_from(0.16449340668482264365e-1_f64) * t98425 - F::cast_from(0.16449340668482264365e-1_f64) * t98428 + F::cast_from(0.16449340668482264365e-1_f64) * t98432 - F::cast_from(0.82246703342411321825e-2_f64) * t98435 + F::cast_from(0.3289868133696452873e-1_f64) * t98439 + F::cast_from(0.3289868133696452873e-1_f64) * t98443 - F::cast_from(0.16449340668482264365e-1_f64) * t98446 - t812 * t22992 * t5617;
    t98450
}
