//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1291/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1291<F: Float>(t1983: F, t33335: F, t6999: F, t8606: F, t8944: F, t26164: F, t33211: F, t7057: F, t649: F, t7467: F, t2040: F, t33363: F, t7000: F, t115774: F, t7687: F, t1307: F, t22574: F, t26558: F, t33221: F) -> (F, F, F, F, F, F, F, F) {
    let t122645 = t1983 * t33335 * t6999;
    let t122654 = t8606 * t8944;
    let t122656 = 2.0 * t122654 * t26164;
    let t122659 = 2.0 * t33211 * t7057;
    let t122660 = t649 * t7467;
    let t122662 = 2.0 * t122660 * t2040;
    let t122664 = t33363 * t7000;
    let t122667 = 3.0 * t1983 * t115774 * t7687;
    let t122671 = 6.0 * t22574 * t26558 * t33221 * t1307;
    (t122645, t122656, t122659, t122660, t122662, t122664, t122667, t122671)
}
