//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 765/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk765<F: Float>(t2940: F, t961: F, t2904: F, t2906: F, t951: F, t959: F, t2924: F, t942: F, t2929: F, t2932: F, t2262: F, t338: F) -> (F, F, F, F, F, F, F, F) {
    let t2942 = F::cast_from(0.11696447245269292414e1_f64) * t2940 * t961;
    let t2944 = t2904 * t2906 * t951;
    let t2946 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t2944;
    let t2948 = t942 * t2924 * t951;
    let t2950 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t2948;
    let t2951 = t2929 * t2906;
    let t2952 = t2951 * t2932;
    let t2954 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t2952;
    let t2955 = t2262 * t338;
    (t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955)
}
