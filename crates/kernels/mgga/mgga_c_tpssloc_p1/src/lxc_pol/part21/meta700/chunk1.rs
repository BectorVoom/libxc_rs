//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2529/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2529<F: Float>(t10213: F, t134: F, t344: F, t13537: F, t2986: F, t4509: F, t4540: F, t13797: F, t1597: F, t10186: F, t13848: F, t13780: F) -> (F, F, F, F, F, F) {
    let t48213 = t134 * t10213 * t344;
    let t48215 = t2986 * t48213 * t13537;
    let t48217 = t4509 * t4540;
    let t48221 = t13797 * t1597;
    let t48233 = t10186 * t13848;
    let t48242 = t10186 * t13780;
    (t48213, t48215, t48217, t48221, t48233, t48242)
}
