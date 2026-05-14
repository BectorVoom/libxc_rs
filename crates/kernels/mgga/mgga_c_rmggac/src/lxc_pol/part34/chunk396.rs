//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 396/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk396<F: Float>(t2100: F, t8705: F, t3826: F, t8625: F, t3810: F, t8631: F, t7583: F, t8702: F, t8706: F, t8710: F, t8714: F, t8716: F, t8718: F, t8720: F, t797: F, t8700: F) -> (F, F, F, F, F) {
    let t8722 = t2100 * t8705;
    let t8724 = t3826 * t8625;
    let t8726 = t3810 * t8631;
    let t8728 = -0.45457769423742236216e-2 * t8702 + 0.9072038638458063915e-4 * t8706 - 0.2419210303588817044e-3 * t8710 + 0.28224120208536198848e-3 * t8714 - 0.90915538847484472432e-2 * t8716 + 0.12122071846331262991e-1 * t8718 - 0.10584045078201074568e-3 * t8720 + 0.34093327067806677162e-2 * t8722 + 0.19914231157590872008e-2 * t8724 - 0.27879923620627220811e-2 * t8726 + t7583;
    let t8729 = t797 * t8700;
    (t8722, t8724, t8726, t8728, t8729)
}
