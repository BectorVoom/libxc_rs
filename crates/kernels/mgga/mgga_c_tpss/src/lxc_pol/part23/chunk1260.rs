//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1260/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1260<F: Float>(t19597: F, t3499: F, t3166: F, t6112: F, t626: F, t17907: F, t6103: F, t13133: F, t5532: F, t1689: F, t42336: F, t13554: F, t5522: F, t18404: F, t6106: F, t7798: F) -> (F, F, F, F, F, F, F, F) {
    let t63725 = 4.0 * t3499 * t19597;
    let t63728 = 2.0 * t626 * t3166 * t6112;
    let t63730 = 2.0 * t6103 * t17907;
    let t63740 = 4.0 * t13133 * t5532;
    let t63742 = 2.0 * t42336 * t1689;
    let t63744 = 4.0 * t13554 * t5522;
    let t63746 = 2.0 * t6103 * t18404;
    let t63748 = 2.0 * t7798 * t6106;
    (t63725, t63728, t63730, t63740, t63742, t63744, t63746, t63748)
}
