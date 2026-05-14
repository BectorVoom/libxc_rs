//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1212/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1212<F: Float>(t23880: F, t26542: F, t26545: F, t75795: F, t8319: F, t1395: F, t1458: F, t1437: F, t1862: F, t645: F, t8308: F, t113875: F, t641: F, t1409: F, t83817: F, t31682: F, t3966: F) -> (F, F, F, F, F, F, F, F) {
    let t120838 = t23880 * t26542;
    let t120840 = t23880 * t26545;
    let t120848 = 27.0 * t75795 * t8319;
    let t120849 = t1395 * t1458;
    let t120851 = 27.0 * t120849 * t8319;
    let t121022 = t1862 * t1437;
    let t121024 = t8308 * t121022 * t645;
    let t121032 = t113875 * t121022 * t641;
    let t121040 = t8308 * t83817 * t1409;
    let t121044 = t8308 * t31682 * t3966;
    (t120838, t120840, t120848, t120851, t121024, t121032, t121040, t121044)
}
