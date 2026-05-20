//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2357/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2357<F: Float>(t2986: F, t344: F, t43052: F, t4343: F, t2978: F, t4338: F, t697: F, t43053: F, t4514: F, t13542: F, t13779: F, t13546: F) -> (F, F, F, F, F) {
    let t48373 = t2986 * t43052 * t344 * t4343;
    let t48374 = F::cast_from(0.37037037037037037036e-3_f64) * t48373;
    let t48378 = t2986 * t697 * t2978 * t344 * t4338;
    let t48379 = F::cast_from(0.24691358024691358024e-3_f64) * t48378;
    let t48381 = t2986 * t43053 * t4514;
    let t48382 = F::cast_from(0.18518518518518518518e-3_f64) * t48381;
    let t48384 = t2986 * t13779 * t13542;
    let t48387 = t2986 * t13779 * t13546;
    (t48374, t48379, t48382, t48384, t48387)
}
