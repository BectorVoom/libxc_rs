//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1030/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1030<F: Float>(t23384: F, t30869: F, t30796: F, t6680: F, t1036: F, t30824: F, t23472: F, t6746: F, t6753: F, t1940: F, t23478: F, t23477: F, t30829: F, t3103: F, t30828: F, t3113: F) -> (F, F, F, F, F, F, F, F) {
    let t113314 = t23384 * t30869;
    let t113318 = t6680 * t30796;
    let t113361 = t30824 * t1036;
    let t113372 = t23472 * t6753 * t6746;
    let t113380 = t23478 * t1940;
    let t113381 = t23477 * t113380;
    let t113388 = t30829 * t3103;
    let t113392 = t3113 * t30828;
    (t113314, t113318, t113361, t113372, t113380, t113381, t113388, t113392)
}
