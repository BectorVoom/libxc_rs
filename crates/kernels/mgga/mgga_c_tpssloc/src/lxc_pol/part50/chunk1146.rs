//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1146/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1146<F: Float>(t23880: F, t26542: F, t26545: F, t112: F, t33164: F, t75795: F, t8319: F, t1395: F, t1458: F, t114475: F, t114495: F, t120815: F, t120818: F, t120820: F, t120823: F, t120826: F, t120830: F, t120835: F, t120836: F, t31267: F, t31287: F, t33192: F, t4072: F, t5376: F, t671: F) -> (F,) {
    let t120838 = t23880 * t26542;
    let t120840 = t23880 * t26545;
    let t120842 = t33164 * t112;
    let t120848 = 27.0 * t75795 * t8319;
    let t120849 = t1395 * t1458;
    let t120851 = 27.0 * t120849 * t8319;
    let t120852 = 27.0 * t120815 + t120818 + t120820 + t120823 + 0.135e2 * t31267 * t4072 + 27.0 * t120826 + t120830 + 27.0 * t114495 * t5376 + t31287 + t120835 + 54.0 * t120836 + 54.0 * t120838 + 54.0 * t120840 + 0.135e2 * t120842 * t671 + t33192 + 0.135e2 * t114475 * t1458 + t120848 + t120851;
    (t120852,)
}
