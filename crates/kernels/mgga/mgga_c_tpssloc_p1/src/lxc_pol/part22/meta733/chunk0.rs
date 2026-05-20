//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2404/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404<F: Float>(t13515: F, t5727: F, t17423: F, t4354: F, t49269: F, t5730: F, t21268: F, t42143: F, t21300: F, t2787: F, t47705: F, t47707: F, t48103: F, t48919: F, t48924: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F) -> (F, F, F, F, F, F) {
    let t68767 = F::new(3.0) * t13515 * t5727;
    let t68769 = F::new(3.0) * t4354 * t17423;
    let t68771 = F::cast_from(0.48245938496077605201e2_f64) * t49269 * t5730;
    let t68773 = F::cast_from(0.96491876992155210402e2_f64) * t42143 * t21268;
    let t68775 = F::new(1.0) * t2787 * t21300;
    let t68785 = F::cast_from(0.59793333333333333333e0_f64) * t68442 + F::cast_from(0.99655555555555555557e-1_f64) * t68444 + F::cast_from(0.11072839506172839506e0_f64) * t68446 - F::cast_from(0.39862222222222222223e0_f64) * t68448 + F::cast_from(0.79724444444444444446e0_f64) * t47705 - F::cast_from(0.26574814814814814815e0_f64) * t47707 - t48919 - t48924 - F::cast_from(0.32862666666666666666e0_f64) * t68452 + F::cast_from(0.54771111111111111112e-1_f64) * t68454 + F::cast_from(0.73028148148148148149e0_f64) * t48103;
    (t68767, t68769, t68771, t68773, t68775, t68785)
}
