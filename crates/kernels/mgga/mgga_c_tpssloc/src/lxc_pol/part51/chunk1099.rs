//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1099/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1099<F: Float>(t1992: F, t33250: F, t1807: F, t8617: F, t31576: F, t31578: F, t31582: F, t32712: F, t32715: F, t32718: F, t32722: F, t32724: F, t539: F, t2016: F, t27068: F, t31106: F, t31113: F, t31115: F, t31596: F, t32700: F, t32707: F, t32733: F, t32737: F, t568: F) -> (F, F, F, F, F) {
    let t33251 = t1992 * t33250;
    let t33259 = t1807 * t8617;
    let t33266 = -t31576 - 0.96894614625936938046e-2 * t32712 - t31578 - 0.16149102437656156341e-2 * t32715 + t32718 / 768.0 - t32722 / 768.0 - t31582 - t32724 / 192.0;
    let t33267 = t539 * t33266;
    let t33269 = -t2016 * t27068 + t33259 * t568 + t33267 * t568 - t31106 - t31113 + t31115 + t31596 - t32700 + t32707 - t32733 - t32737;
    (t33251, t33259, t33266, t33267, t33269)
}
