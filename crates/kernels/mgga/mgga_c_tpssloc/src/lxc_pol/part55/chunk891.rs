//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 891/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk891<F: Float>(t26388: F, t6637: F, t6888: F, t22893: F, t7732: F, t22892: F, t1834: F, t552: F, t1307: F, t26328: F, t553: F, t1824: F, t2006: F, t1352: F, t6914: F, t7737: F) -> (F, F, F, F, F, F, F) {
    let t26389 = t6637 * t26388;
    let t26390 = t6888 * t26389;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26396 = t26395 * t1307;
    let t26397 = t6637 * t26396;
    let t26398 = t6888 * t26397;
    let t26401 = t553 * t26328;
    let t26403 = t2006 * t1824;
    let t26404 = t26403 * t1352;
    let t26406 = t6914 * t7737;
    (t26390, t26393, t26398, t26401, t26403, t26404, t26406)
}
