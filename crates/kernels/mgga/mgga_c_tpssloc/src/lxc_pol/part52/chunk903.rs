//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 903/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk903<F: Float>(t23600: F, t350: F, t3030: F, t344: F, t225: F, t6733: F, t1949: F, t2966: F, t1920: F, t6680: F, t6781: F, t6805: F, t968: F, t210: F, t6795: F, t6688: F, t974: F) -> (F, F, F, F, F, F, F, F) {
    let t23601 = t23600 * t350;
    let t23602 = t344 * t3030;
    let t23613 = t6733 * t225;
    let t23617 = t2966 * t1949;
    let t23619 = 0.18277045187202515961e-2 * t1920 * t23617;
    let t23626 = t6680 * t6781;
    let t23628 = t968 * t6805;
    let t23629 = t1920 * t23628;
    let t23631 = t6795 * t210;
    let t23632 = t974 * t6688;
    (t23601, t23602, t23613, t23619, t23626, t23629, t23631, t23632)
}
