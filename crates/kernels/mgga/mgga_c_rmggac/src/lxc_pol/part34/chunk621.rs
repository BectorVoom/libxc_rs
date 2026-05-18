//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 621/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk621<F: Float>(t15164: F, t15167: F, t15170: F, t15172: F, t14516: F, t2344: F, t14509: F, t2329: F, t14512: F, t2333: F, t15526: F, t305: F) -> (F, F, F, F, F, F, F, F) {
    let t15559 = F::new(0.20455996240684006298e-1) * t15164;
    let t15560 = F::new(0.2727466165424534173e-1) * t15167;
    let t15561 = F::new(0.13637330827122670865e-1) * t15170;
    let t15562 = F::new(0.14967802127329760705e-1) * t15172;
    let t15563 = t14516 * t2344;
    let t15564 = F::new(0.10227998120342003148e-1) * t15563;
    let t15565 = t14509 * t2329;
    let t15566 = F::new(0.13637330827122670864e-1) * t15565;
    let t15567 = t14512 * t2333;
    let t15568 = F::new(0.68186654135613354322e-2) * t15567;
    let t15570 = t305 * t15526;
    (t15559, t15560, t15561, t15562, t15564, t15566, t15568, t15570)
}
