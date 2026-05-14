//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 963/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk963<F: Float>(t13831: F, t4531: F, t10254: F, t3961: F, t2988: F, t10236: F, t10235: F, t10186: F, t10233: F, t10267: F, t10274: F, t13806: F, t13813: F, t13817: F, t13825: F, t13830: F, t2960: F, t2986: F, t4523: F, t4532: F, t4549: F, t973: F) -> (F,) {
    let t13832 = t4531 * t13831;
    let t13835 = t10254 * t3961;
    let t13836 = t2988 * t13835;
    let t13839 = t10236 * t3961;
    let t13840 = t10235 * t13839;
    let t13845 = 0.12345679012345679012e-3 * t10233 + 0.55555555555555555554e-3 * t2986 * t13806 - 0.49382716049382716048e-3 * t10267 - 0.18518518518518518518e-3 * t10274 - 0.16666666666666666666e-2 * t973 * t13813 + 0.27777777777777777777e-3 * t973 * t13817 + 0.44444444444444444444e-2 * t2960 * t4549 - t13825 - 0.14814814814814814814e-2 * t2960 * t4523 + t13830 - 0.55555555555555555554e-3 * t2986 * t13832 + 0.11111111111111111111e-2 * t2986 * t13836 - 0.74074074074074074072e-3 * t2986 * t13840 + 0.14814814814814814814e-2 * t10186 * t4532;
    (t13845,)
}
