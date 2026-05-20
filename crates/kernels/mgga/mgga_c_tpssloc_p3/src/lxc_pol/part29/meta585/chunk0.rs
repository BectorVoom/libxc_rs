//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2006/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2006<F: Float>(t22773: F, t22779: F, t22865: F, t6604: F, t6937: F, t22776: F, t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F) -> (F, F, F, F, F, F, F) {
    let t80922 = t22779 * t22773;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80943 = t22779 * t22776;
    let t80953 = F::new(1.0) / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80957 = F::cast_from(0.69792532988666768264e-2_f64) * t80956;
    let t80958 = t22803 * t6604;
    (t80922, t80939, t80940, t80943, t80953, t80957, t80958)
}
