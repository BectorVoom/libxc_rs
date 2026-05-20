//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2573/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2573<F: Float>(t14740: F, t15419: F, t3447: F, t11584: F, t15338: F, t44583: F, t461: F, t4729: F, t15418: F, t1714: F, t11571: F, t14736: F) -> (F, F, F, F, F) {
    let t52050 = t3447 * t15419 * t14740;
    let t52053 = t3447 * t15338 * t11584;
    let t52057 = t3447 * t44583 * t461 * t4729;
    let t52058 = F::cast_from(0.37037037037037037036e-3_f64) * t52057;
    let t52059 = t15418 * t1714;
    let t52061 = t3447 * t52059 * t11571;
    let t52064 = t3447 * t15419 * t14736;
    (t52050, t52053, t52058, t52061, t52064)
}
