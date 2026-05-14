//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 770/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk770<F: Float>(t29514: F, t29847: F, t2165: F, t5493: F, t113: F, t1442: F, t1774: F, t28815: F, t28819: F, t28822: F, t28825: F, t28829: F, t28833: F, t28837: F, t28841: F, t28843: F, t28861: F, t28863: F, t28866: F, t29493: F, t4028: F, t510: F, t5450: F, t5457: F, t652: F, t7983: F, t7989: F, t8103: F) -> (F, F, F) {
    let t29848 = t29514 + t29847;
    let t29855 = t2165 * t5493;
    let t29864 = -t113 * t29848 - 2.0 * t1442 * t8103 - 2.0 * t1774 * t7983 - t2165 * t5450 - 2.0 * t2165 * t5457 - 2.0 * t29493 * t510 - 2.0 * t29855 * t652 - 4.0 * t4028 * t7989 - t28815 + t28819 + t28822 + t28825 + t28829 - t28833 + t28837 + t28841 + t28843 - t28861 - t28863 - t28866;
    (t29848, t29855, t29864)
}
