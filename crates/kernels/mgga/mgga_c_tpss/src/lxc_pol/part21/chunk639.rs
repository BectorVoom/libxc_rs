//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 639/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk639<F: Float>(t2785: F, t345: F, t220: F, t2768: F, t2782: F, t2783: F, t2786: F, t2790: F, t2794: F, t2798: F, t368: F, t983: F, t985: F, t981: F, t2769: F, t2771: F, t2778: F, t373: F, t978: F, t991: F) -> (F, F, F, F) {
    let t2799 = t2785 * t345;
    let t2804 = t220 * t2768 * t368 + 2.0 * t2782 * t2783 * t2786 - t2783 * t2798 * t2799 + 2.0 * t2790 * t983 * t985 + t2794 * t983 * t985;
    let t2805 = t981 * t2804;
    let t2807 = t2769 * t373 - 2.0 * t2771 * t991 + 2.0 * t2778 * t978 - t2805 * t978;
    (t2799, t2804, t2805, t2807)
}
