//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 639/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk639<F: Float>(t2841: F, t2895: F, t141: F, t1038: F, t2846: F, t2850: F, t2836: F, t2843: F, t2848: F, t2852: F, t2870: F, t2878: F, t2880: F, t2886: F, t2888: F, t2892: F, t2893: F) -> (F, F, F, F, F, F, F) {
    let t2896 = t2895 * t2841;
    let t2897 = t141 * t2896;
    let t2899 = t1038 * t2846;
    let t2900 = t141 * t2899;
    let t2902 = t1038 * t2850;
    let t2903 = t141 * t2902;
    let t2905 = -F::new(0.9494625e0) * t2870 + F::new(0.1898925e1) * t2878 + t2880 - F::new(0.19931111111111111111e0) * t2836 - F::new(0.19931111111111111111e0) * t2843 + F::new(0.59793333333333333334e0) * t2848 + F::new(0.29896666666666666667e0) * t2852 + F::new(0.15358125e0) * t2886 + F::new(0.3071625e0) * t2888 + t2892 - F::new(0.10954222222222222222e0) * t2893 - F::new(0.27385555555555555556e-1) * t2897 + F::new(0.16431333333333333333e0) * t2900 + F::new(0.82156666666666666667e-1) * t2903;
    (t2896, t2897, t2899, t2900, t2902, t2903, t2905)
}
