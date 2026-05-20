//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1983/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1983<F: Float>(t87432: F, t87443: F, t81918: F, t81924: F, t81926: F, t81928: F, t81934: F, t81936: F, t81943: F, t84921: F, t87418: F, t87422: F, t87425: F, t87428: F, t87430: F, t87445: F, t87449: F, t87453: F) -> F {
    let t92689 = F::cast_from(0.22608743412718618878e-1_f64) * t87432;
    let t92697 = F::cast_from(0.80745512188280781706e-3_f64) * t87443;
    let t92701 = F::cast_from(0.33913115119077928316e-1_f64) * t87418 - t87422 / F::new(2.0) - F::cast_from(0.23739180583354549822e0_f64) * t87425 + F::cast_from(0.16956557559538964158e-1_f64) * t87428 - t87430 / F::new(24.0) - t92689 - F::cast_from(0.13457585364713463618e-3_f64) * t81918 - t84921 + F::cast_from(0.67287926823567318088e-4_f64) * t81924 - F::new(7.0) / F::new(1152.0) * t81926 + F::new(119.0) / F::new(1728.0) * t81928 - F::cast_from(0.27130492095262342653e0_f64) * t81934 + F::cast_from(0.16956557559538964158e-1_f64) * t81936 - F::new(35.0) / F::new(54.0) * t81943 + t92697 + F::cast_from(0.20186378047070195426e-3_f64) * t87445 - F::cast_from(0.33913115119077928316e-1_f64) * t87449 + F::cast_from(0.48447307312968469024e-2_f64) * t87453;
    t92701
}
