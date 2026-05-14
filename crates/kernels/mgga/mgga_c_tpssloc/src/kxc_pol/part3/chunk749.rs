//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 749/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk749<F: Float>(t4613: F, t4656: F, t349: F, t1626: F, t225: F, t1065: F, t1634: F, t3174: F, t1057: F, t4639: F, t1022: F, t3188: F, t1629: F, t1049: F, t1615: F, t1060: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4657 = t4613 + t4656;
    let t4658 = t349 * t4657;
    let t4660 = t1626 * t225;
    let t4664 = t1634 * t1065;
    let t4665 = t3174 * t4664;
    let t4669 = t4639 * t1057;
    let t4673 = t3188 * t1022;
    let t4674 = t1629 * t4673;
    let t4677 = t1049 * t1615;
    let t4678 = t4677 * t1060;
    (t4657, t4658, t4660, t4665, t4669, t4673, t4674, t4677, t4678)
}
