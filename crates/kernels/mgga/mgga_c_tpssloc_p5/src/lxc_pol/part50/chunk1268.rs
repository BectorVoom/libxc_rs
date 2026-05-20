//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1268/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1268<F: Float>(t120521: F, t114116: F, t114121: F, t114104: F, t114119: F, t114130: F, t120425: F, t120491: F, t120496: F, t120502: F, t120505: F, t120506: F, t120507: F, t120513: F, t120515: F, t120516: F, t1332: F, t1336: F, t1352: F, t1825: F, t32753: F, t32755: F, t3777: F, t5250: F, t5334: F, t5344: F, t544: F, t553: F) -> F {
    let t120522 = F::cast_from(0.82246703342411321825e-2_f64) * t120521;
    let t120525 = F::cast_from(0.38381794893125283518e-1_f64) * t114116;
    let t120526 = F::cast_from(0.82246703342411321825e-2_f64) * t114121;
    let t120528 = -t114130 * t1336 * t1825 + t120425 * t544 * t553 - t120516 * t1352 * t5344 + F::new(2.0) * t120516 * t5250 * t5334 + t1332 * t32755 - t32753 * t3777 + t114104 + t114119 + t120491 - t120496 - t120502 + t120505 - t120506 + t120507 + t120513 - t120515 - t120522 - t120525 + t120526;
    t120528
}
