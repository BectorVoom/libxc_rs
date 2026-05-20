//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1353/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1353<F: Float>(t33222: F, t91669: F, t33358: F, t83886: F, t24987: F, t8641: F, t120705: F, t22574: F, t24432: F, t31295: F, t7685: F, t19577: F, t36740: F) -> (F, F, F, F, F, F) {
    let t120885 = F::new(2.0) * t91669 * t33222;
    let t120887 = F::new(3.0) * t83886 * t33358;
    let t120888 = t24987 * t8641;
    let t120891 = F::new(3.0) * t22574 * t24432 * t120705;
    let t120892 = t7685 * t31295;
    let t120896 = F::new(3.0) * t22574 * t36740 * t19577;
    (t120885, t120887, t120888, t120891, t120892, t120896)
}
