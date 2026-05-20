//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 813/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk813<F: Float>(t5: F, t109: F, t28941: F, t112: F, t23912: F, t26127: F, t28012: F, t28014: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t110 = F::new(1.0) < t109;
    let t28942 = piecewise3::<F>(t8, F::new(0.0), t28941);
    let t28943 = t28942 * t112;
    let t28951 = piecewise3::<F>(t110, F::new(0.0), t23912 + F::new(4.0) / F::new(3.0) * t26127 + t28012 / F::new(2.0) - t28014 / F::new(4.0));
    (t28942, t28943, t28951)
}
