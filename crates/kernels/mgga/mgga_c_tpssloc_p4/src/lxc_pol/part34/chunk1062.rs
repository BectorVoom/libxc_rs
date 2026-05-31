//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1062/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1062<F: Float>(t5: F, t109: F, t28941: F, t112: F, t23912: F, t26127: F, t28012: F, t28014: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t110 = F::cast_from(1.0_f64) < t109;
    let t28942 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t28941);
    let t28943 = t28942 * t112;
    let t28951 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t23912 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26127 + t28012 / F::cast_from(2.0_f64) - t28014 / F::cast_from(4.0_f64));
    (t28942, t28943, t28951)
}
