//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 715/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk715<F: Float>(t9924: F, t185: F, t9258: F, t707: F, t32: F, t717: F, t2659: F, t2244: F, t751: F, t2658: F, t9853: F, t9859: F, t9911: F, t9914: F, t9917: F, t9921: F, t9923: F) -> (F, F, F, F, F) {
    let t9925 = 24.0 * t9924;
    let t9926 = t185 * t9258;
    let t9928 = 4.0 * t707 * t9926;
    let t9929 = t32 * t717;
    let t9931 = 36.0 * t9929 * t2659;
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9934 = 36.0 * t9933;
    let t9935 = t9853 + t9911 + t9914 + t9917 - t9921 - t9923 + t9925 + t9859 + t9928 + t9931 + t9934;
    (t9925, t9928, t9931, t9934, t9935)
}
