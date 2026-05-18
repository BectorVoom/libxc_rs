//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 577/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk577<F: Float>(t14252: F, t14256: F, t14259: F, t2020: F, t3180: F, t2019: F, t2604: F, t3188: F, t14494: F, t515: F, t235: F, t14375: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14693 = F::new(0.23268647941669485538e-4) * t14252;
    let t14694 = F::new(0.58171619854173713846e-5) * t14256;
    let t14695 = F::new(0.58171619854173713846e-5) * t14259;
    let t14696 = t2020 * t3180;
    let t14697 = t2019 * t14696;
    let t14701 = t2604 * t3188;
    let t14702 = F::new(0.14967802127329760705e-1) * t14701;
    let t14703 = t515 * t14494;
    let t14704 = t235 * t14703;
    let t14705 = F::new(0.19957069503106347607e-1) * t14704;
    let t14709 = F::new(0.1276937996798935182e-4) * t14375;
    (t14693, t14694, t14695, t14696, t14697, t14702, t14703, t14705, t14709)
}
