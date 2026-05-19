//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 622/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk622<F: Float>(t15624: F, t515: F, t1971: F, t7230: F, t14581: F, t2344: F, t14585: F, t2329: F, t14589: F, t2333: F, t15311: F, t15315: F) -> (F, F, F, F, F, F, F) {
    let t15625 = t515 * t15624;
    let t15626 = t1971 * t15625;
    let t15627 = t7230 * t15626;
    let t15628 = F::cast_from(0.53205749866622299248e-5_f64) * t15627;
    let t15632 = t14581 * t2344;
    let t15633 = F::cast_from(0.10227998120342003148e-1_f64) * t15632;
    let t15634 = t14585 * t2329;
    let t15635 = F::cast_from(0.13637330827122670864e-1_f64) * t15634;
    let t15636 = t14589 * t2333;
    let t15637 = F::cast_from(0.68186654135613354322e-2_f64) * t15636;
    let t15640 = F::cast_from(0.10227998120342003148e-1_f64) * t15311;
    let t15643 = F::cast_from(0.40911992481368012592e-1_f64) * t15315;
    (t15626, t15628, t15633, t15635, t15637, t15640, t15643)
}
