//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 636/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk636<F: Float>(t1043: F, t2863: F, t2862: F, t392: F, t395: F, t1027: F, t2834: F, t2836: F, t2843: F, t2848: F, t2852: F, t1025: F) -> (F, F, F, F, F, F, F, F) {
    let t2864 = t2863 * t1043;
    let t2866 = F::new(2.0) * t2862 * t2864;
    let t2868 = F::new(1.0) / t395 / t392;
    let t2869 = t1027 * t1027;
    let t2870 = t2868 * t2869;
    let t2872 = F::new(4.0) / F::new(9.0) * t2834;
    let t2877 = t2872 - F::new(2.0) / F::new(9.0) * t2836 - F::new(2.0) / F::new(9.0) * t2843 + F::new(2.0) / F::new(3.0) * t2848 + t2852 / F::new(3.0);
    let t2878 = t1025 * t2877;
    (t2864, t2866, t2868, t2869, t2870, t2872, t2877, t2878)
}
