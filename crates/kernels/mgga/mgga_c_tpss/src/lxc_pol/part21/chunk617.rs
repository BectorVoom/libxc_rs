//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 617/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk617<F: Float>(t2459: F, t2697: F, t1985: F, t926: F, t2475: F, t2478: F, t2485: F, t2528: F, t2536: F, t2626: F, t2628: F, t2631: F, t2635: F, t2639: F, t2643: F) -> (F, F, F) {
    let t2698 = t2697 * t2459;
    let t2699 = t2698 * t1985;
    let t2700 = t926 * t2699;
    let t2703 = -t2475 + t2478 - t2485 + t2528 + t2536 + t2626 + t2628 - t2631 + t2635 - t2639 - t2643;
    (t2699, t2700, t2703)
}
