//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2946/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946<F: Float>(t10263: F, t13769: F, t13831: F, t17800: F, t2986: F, t4347: F, t4518: F, t4531: F, t48207: F, t48210: F, t48215: F, t48233: F, t48242: F, t48244: F, t48250: F, t48256: F, t5839: F, t59767: F, t6733: F) -> F {
    let t61355 = -F::cast_from(0.81481481481481481481e-2_f64) * t10263 * t5839 - F::cast_from(0.55555555555555555554e-3_f64) * t48207 + F::cast_from(0.24691358024691358024e-3_f64) * t48210 + F::cast_from(0.57613168724279835389e-3_f64) * t48215 + F::cast_from(0.98765432098765432095e-3_f64) * t48233 + F::cast_from(0.19753086419753086419e-2_f64) * t48242 - F::cast_from(0.13168724279835390946e-2_f64) * t48244 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17800 * t13831 - F::cast_from(0.14814814814814814814e-2_f64) * t2986 * t13769 * t48256 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4531 * t6733 * t4347 - F::cast_from(0.49382716049382716048e-3_f64) * t48250 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4518 * t59767;
    t61355
}
