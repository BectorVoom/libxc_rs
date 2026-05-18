//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 988/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk988<F: Float>(t305: F, t77871: F, t14516: F, t8537: F, t2471: F, t838: F, t2141: F, t326: F, t9530: F, t2147: F, t76197: F, t76199: F) -> (F, F, F, F, F, F) {
    let t77872 = t305 * t77871;
    let t77873 = F::new(0.14967802127329760705e-1) * t77872;
    let t77874 = t14516 * t8537;
    let t77875 = F::new(0.27274661654245341728e-1) * t77874;
    let t77876 = t838 * t2471;
    let t77877 = t77876 * t2141;
    let t77878 = F::new(0.13637330827122670864e-1) * t77877;
    let t77879 = t326 * t9530;
    let t77880 = t77879 * t2147;
    let t77881 = F::new(0.68186654135613354322e-2) * t77880;
    let t77883 = F::new(0.17961362552795712846e0) * t76197;
    let t77884 = F::new(0.44903406381989282115e-1) * t76199;
    (t77873, t77875, t77878, t77881, t77883, t77884)
}
