//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1434/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1434<F: Float>(t33240: F, t6883: F, t225: F, t33267: F, t115567: F, t120542: F, t120547: F, t120551: F, t120552: F, t120553: F, t120556: F, t1385: F, t1386: F, t26224: F, t26366: F, t27068: F, t31601: F, t5321: F, t6993: F, t7199: F, t7728: F, t93319: F) -> F {
    let t122295 = t6883 * t33240;
    let t122297 = t33267 * t225;
    let t122299 = -t27068 * t6993 + t120542 + F::new(2.0) * t26366 * t7199 + F::new(24.0) * t26224 * t93319 * t7728 * t1385 - t120547 + F::new(2.0) * t5321 * t31601 - t120551 + t115567 + F::new(0.19190897446562641759e-1) * t122295 - t120552 - t122297 * t1386 + t120553 + t120556;
    t122299
}
