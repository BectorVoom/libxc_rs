//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1113/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1113<F: Float>(t10186: F, t10233: F, t10267: F, t10274: F, t13806: F, t13813: F, t13817: F, t13825: F, t13830: F, t13832: F, t13836: F, t13840: F, t2960: F, t2986: F, t4523: F, t4532: F, t4549: F, t973: F) -> F {
    let t13845 = F::new(0.12345679012345679012e-3) * t10233 + F::new(0.55555555555555555554e-3) * t2986 * t13806 - F::new(0.49382716049382716048e-3) * t10267 - F::new(0.18518518518518518518e-3) * t10274 - F::new(0.16666666666666666666e-2) * t973 * t13813 + F::new(0.27777777777777777777e-3) * t973 * t13817 + F::new(0.44444444444444444444e-2) * t2960 * t4549 - t13825 - F::new(0.14814814814814814814e-2) * t2960 * t4523 + t13830 - F::new(0.55555555555555555554e-3) * t2986 * t13832 + F::new(0.11111111111111111111e-2) * t2986 * t13836 - F::new(0.74074074074074074072e-3) * t2986 * t13840 + F::new(0.14814814814814814814e-2) * t10186 * t4532;
    t13845
}
