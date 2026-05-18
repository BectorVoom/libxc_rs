//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 618/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk618<F: Float>(t15172: F, t14516: F, t2344: F, t14509: F, t2329: F, t14512: F, t2333: F, t15526: F, t305: F, t118: F, t15516: F, t15176: F) -> (F, F, F, F, F, F, F) {
    let t15562 = F::new(0.14967802127329760705e-1) * t15172;
    let t15563 = t14516 * t2344;
    let t15564 = F::new(0.10227998120342003148e-1) * t15563;
    let t15565 = t14509 * t2329;
    let t15566 = F::new(0.13637330827122670864e-1) * t15565;
    let t15567 = t14512 * t2333;
    let t15568 = F::new(0.68186654135613354322e-2) * t15567;
    let t15570 = t305 * t15526;
    let t15571 = F::new(0.14967802127329760705e-1) * t15570;
    let t15573 = F::new(0.39914139006212695214e-1) * t118 * t15516;
    let t15574 = F::new(0.44903406381989282115e-1) * t15176;
    (t15562, t15564, t15566, t15568, t15571, t15573, t15574)
}
