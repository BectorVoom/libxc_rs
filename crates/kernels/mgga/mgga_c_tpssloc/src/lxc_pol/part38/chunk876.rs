//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 876/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk876<F: Float>(t2427: F, t2430: F, t32: F, t717: F, t2244: F, t751: F, t2658: F, t813: F, t236: F, t232: F, t2632: F, t2639: F, t2686: F, t2697: F, t2703: F, t842: F, t9612: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9924 = t2427 * t2430;
    let t9929 = t32 * t717;
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9970 = t813 * t813;
    let t9971 = 1.0 / t9970;
    let t9972 = t9971 * t236;
    let t9975 = t2632 * t232;
    let t9986 = t2639 * t2686;
    let t9988 = t2697 * t2703;
    let t9990 = t9612 * t842;
    (t9924, t9929, t9933, t9971, t9972, t9975, t9986, t9988, t9990)
}
