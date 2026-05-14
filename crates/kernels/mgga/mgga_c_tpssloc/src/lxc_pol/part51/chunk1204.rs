//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1204/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1204<F: Float>(t33222: F, t91669: F, t33358: F, t83886: F, t24987: F, t8641: F, t120705: F, t22574: F, t24432: F, t31295: F, t7685: F, t19577: F, t36740: F, t120874: F, t120876: F, t120877: F, t120878: F, t120881: F, t12725: F, t5361: F, t8529: F, t8604: F) -> (F,) {
    let t120885 = 2.0 * t91669 * t33222;
    let t120887 = 3.0 * t83886 * t33358;
    let t120888 = t24987 * t8641;
    let t120891 = 3.0 * t22574 * t24432 * t120705;
    let t120892 = t7685 * t31295;
    let t120896 = 3.0 * t22574 * t36740 * t19577;
    let t120897 = -2.0 * t12725 * t8529 + t5361 * t8604 + t120874 + t120876 - t120877 - t120878 - t120881 + t120885 - t120887 + t120888 - t120891 - t120892 - t120896;
    (t120897,)
}
