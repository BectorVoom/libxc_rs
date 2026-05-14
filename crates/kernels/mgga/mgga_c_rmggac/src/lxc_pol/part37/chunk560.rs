//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 560/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk560<F: Float>(t15359: F, t15372: F, t15395: F, t15406: F, t2211: F, t8975: F, t739: F, t8946: F, t884: F, t8041: F, t8936: F, t1356: F, t15030: F, t15033: F, t15068: F, t15082: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15656 = 0.14967802127329760705e-1 * t15359;
    let t15660 = 0.23268647941669485538e-4 * t15372;
    let t15665 = 0.1276937996798935182e-4 * t15395;
    let t15667 = 0.85129199786595678799e-5 * t15406;
    let t15669 = t2211 * t8975;
    let t15670 = t739 * t15669;
    let t15671 = 0.11974241701863808564e0 * t15670;
    let t15672 = t2211 * t8946;
    let t15673 = t884 * t15672;
    let t15674 = 0.11974241701863808564e0 * t15673;
    let t15675 = t8041 * t8936;
    let t15676 = t1356 * t15675;
    let t15677 = 0.11974241701863808564e0 * t15676;
    let t15856 = 0.32526727992809621482e-5 * t15030;
    let t15857 = 0.32526727992809621482e-5 * t15033;
    let t15858 = 0.17519306092901367186e-5 * t15068;
    let t15859 = 0.76860658247009135562e-5 * t15082;
    (t15656, t15660, t15665, t15667, t15669, t15671, t15672, t15674, t15675, t15677, t15856, t15857, t15858, t15859)
}
