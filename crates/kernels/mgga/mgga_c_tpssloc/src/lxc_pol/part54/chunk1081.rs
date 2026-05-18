//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1081/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1081<F: Float>(t27095: F, t27113: F, t1378: F, t1375: F, t1386: F, t16022: F, t16439: F, t1843: F, t2092: F, t22676: F, t24095: F, t26475: F, t27067: F, t27068: F, t27070: F, t3758: F, t3882: F, t5215: F, t5321: F, t568: F, t7199: F, t7214: F, t7937: F) -> (F, F, F) {
    let t27114 = t27095 + t27113;
    let t27115 = t1378 * t27114;
    let t27127 = -t27067 - t27068 * t1386 + t27070 * t568 - t1375 * t27115 - t3882 * t7937 - t16022 * t2092 - F::new(0.82246703342411321825e-2) * t26475 - t16439 * t2092 + F::new(2.0) * t5215 * t7199 - t5321 * t7214 - t3758 * t7937 + F::new(0.82246703342411321825e-2) * t22676 - t24095 * t1843;
    (t27114, t27115, t27127)
}
