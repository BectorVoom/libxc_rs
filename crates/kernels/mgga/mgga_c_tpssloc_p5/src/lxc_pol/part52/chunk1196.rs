//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1196/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1196<F: Float>(t32685: F, t3887: F, t1842: F, t8475: F, t12021: F, t31090: F, t22635: F, t1992: F, t6906: F, t7749: F, t6889: F, t1985: F) -> (F, F, F, F, F, F, F, F) {
    let t32686 = t3887 * t32685;
    let t32689 = t8475 * t1842;
    let t32690 = t12021 * t32689;
    let t32693 = t31090 * t1842;
    let t32694 = t22635 * t32693;
    let t32696 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t32694;
    let t32697 = t6906 * t7749;
    let t32698 = t6889 * t32697;
    let t32700 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t32698;
    (t32686, t32690, t32693, t32694, t32696, t32697, t32698, t32700)
}
