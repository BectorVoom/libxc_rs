//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1009/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1009<F: Float>(t26384: F, t6637: F, t6888: F, t5187: F, t6968: F, t22893: F, t7732: F, t22892: F, t1834: F, t552: F, t1307: F, t26328: F, t553: F) -> (F, F, F, F, F) {
    let t26385 = t6637 * t26384;
    let t26386 = t6888 * t26385;
    let t26388 = t6968 * t5187;
    let t26389 = t6637 * t26388;
    let t26390 = t6888 * t26389;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26396 = t26395 * t1307;
    let t26397 = t6637 * t26396;
    let t26398 = t6888 * t26397;
    let t26401 = t553 * t26328;
    (t26386, t26390, t26393, t26398, t26401)
}
