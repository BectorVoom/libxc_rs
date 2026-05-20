//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2954/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2954<F: Float>(t13798: F, t17863: F, t2979: F, t2980: F, t2986: F, t43065: F, t4514: F, t48180: F, t48191: F, t48373: F, t48378: F, t48381: F, t48384: F, t48387: F, t48390: F, t48394: F, t55723: F, t59706: F, t59711: F, t973: F) -> F {
    let t61585 = F::cast_from(0.49382716049382716048e-3_f64) * t48373 - F::cast_from(0.32921810699588477366e-3_f64) * t48378 + F::cast_from(0.24691358024691358024e-3_f64) * t48381 - F::cast_from(0.74074074074074074072e-3_f64) * t48384 - F::cast_from(0.37037037037037037036e-3_f64) * t48387 - F::cast_from(0.14814814814814814814e-2_f64) * t48390 + F::cast_from(0.49382716049382716048e-3_f64) * t48394 + F::cast_from(0.74074074074074074072e-3_f64) * t973 * t2979 * t2980 * t55723 + F::cast_from(0.86419753086419753084e-3_f64) * t2986 * t13798 * t59706 + F::cast_from(0.28806584362139917695e-2_f64) * t2986 * t48180 * t59711 - F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t43065 * t17863 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t48191 * t4514;
    t61585
}
