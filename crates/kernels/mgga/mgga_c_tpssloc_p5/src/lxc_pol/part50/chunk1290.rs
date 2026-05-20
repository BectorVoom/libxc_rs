//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1290/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1290<F: Float>(t120833: F, t8319: F, t7769: F, t96351: F, t23880: F, t26542: F, t26545: F, t112: F, t33164: F, t75795: F, t1395: F, t1458: F) -> (F, F, F, F, F, F, F) {
    let t120835 = F::new(27.0) * t120833 * t8319;
    let t120836 = t96351 * t7769;
    let t120838 = t23880 * t26542;
    let t120840 = t23880 * t26545;
    let t120842 = t33164 * t112;
    let t120848 = F::new(27.0) * t75795 * t8319;
    let t120849 = t1395 * t1458;
    (t120835, t120836, t120838, t120840, t120842, t120848, t120849)
}
