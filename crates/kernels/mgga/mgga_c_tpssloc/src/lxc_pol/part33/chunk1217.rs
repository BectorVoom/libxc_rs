//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1217/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1217<F: Float>(t80825: F, t557: F, t6546: F, t1365: F, t1878: F, t22813: F, t6924: F, t80782: F, t22843: F, t281: F, t6597: F, t154: F, t8705: F) -> (F, F, F, F, F, F) {
    let t80826 = F::cast_from(0.10173934535723378495e0_f64) * t80825;
    let t80827 = t6546 * t557;
    let t80830 = t1878 * t1365;
    let t80836 = t22813 * t6924 * t80782;
    let t80840 = t6597 * t22843 * t281;
    let t80845 = t8705 * t154;
    (t80826, t80827, t80830, t80836, t80840, t80845)
}
