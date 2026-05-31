//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 620/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk620<F: Float>(t2804: F, t981: F, t2769: F, t2771: F, t2778: F, t373: F, t978: F, t991: F, t993: F, t375: F, t198: F, t2475: F, t2478: F, t2485: F, t2528: F, t2536: F, t2626: F, t2628: F, t2631: F, t2635: F, t2639: F, t2643: F, t330: F, t995: F) -> (F, F, F, F, F, F) {
    let t2805 = t981 * t2804;
    let t2807 = t2769 * t373 - F::cast_from(2.0_f64) * t2771 * t991 + F::cast_from(2.0_f64) * t2778 * t978 - t2805 * t978;
    let t2811 = t993 * t993;
    let t2813 = t375 * t375;
    let t2814 = F::cast_from(1.0_f64) / t2813;
    let t2817 = t198 * t2807 * t330 * t995 - t198 * t2811 * t2814 * t330 - t2475 + t2478 - t2485 + t2528 + t2536 + t2626 + t2628 - t2631 + t2635 - t2639 - t2643;
    (t2805, t2807, t2811, t2813, t2814, t2817)
}
