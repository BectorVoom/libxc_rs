//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3125/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125<F: Float>(t4899: F, t6138: F, t6144: F, t11571: F, t15313: F, t15320: F, t15376: F, t15396: F, t3447: F, t4904: F, t4919: F, t51948: F, t51961: F, t51970: F, t51980: F, t51988: F, t51991: F, t51995: F, t52040: F) -> F {
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64660 = F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t52040 * t4904 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t15320 * t15313 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4919 * t51961 - F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t64644 * t11571 - F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t64648 * t11571 + F::cast_from(0.46090534979423868311e-2_f64) * t15376 * t15396 + F::cast_from(0.14814814814814814814e-2_f64) * t51948 - F::cast_from(0.24691358024691358024e-3_f64) * t51970 - F::cast_from(0.24691358024691358024e-3_f64) * t51980 + F::cast_from(0.18518518518518518518e-3_f64) * t51988 - F::cast_from(0.37037037037037037036e-3_f64) * t51991 - F::cast_from(0.11111111111111111111e-2_f64) * t51995;
    t64660
}
