//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1362/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1362<F: Float>(t1873: F, t26135: F, t3941: F, t4072: F, t576: F, t8319: F, t7769: F, t96351: F, t23880: F, t26542: F, t26545: F, t75795: F) -> (F, F, F, F, F, F) {
    let t120830 = F::new(54.0) * t3941 * t1873 * t26135;
    let t120833 = t576 * t4072;
    let t120835 = F::new(27.0) * t120833 * t8319;
    let t120836 = t96351 * t7769;
    let t120838 = t23880 * t26542;
    let t120840 = t23880 * t26545;
    let t120848 = F::new(27.0) * t75795 * t8319;
    (t120830, t120835, t120836, t120838, t120840, t120848)
}
