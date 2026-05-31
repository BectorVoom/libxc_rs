//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 663/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk663<F: Float>(t3: F, t8843: F, t2039: F, t577: F, t7423: F, t8508: F, t8654: F, t8659: F, t192: F, t533: F, t2229: F, t2233: F, t604: F) -> (F, F, F, F, F, F) {
    let t8844 = t3 * t8843;
    let t8852 = F::cast_from(0.45e1_f64) * t8843 * t577 + F::cast_from(0.135e2_f64) * t7423 * t2039 + t8654 + t8659 + t8508;
    let t8944 = t192 * t533;
    let t9222 = t2229 * t3;
    let t9223 = F::cast_from(1.0_f64) / t9222;
    let t9228 = t2233 * t604;
    (t8844, t8852, t8944, t9222, t9223, t9228)
}
