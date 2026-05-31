//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1353/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1353<F: Float>(t22574: F, t31299: F, t33899: F, t33222: F, t91669: F, t33358: F, t83886: F, t24987: F, t8641: F, t120705: F, t24432: F, t31295: F, t7685: F) -> (F, F, F, F, F, F) {
    let t120881 = F::cast_from(3.0_f64) * t22574 * t33899 * t31299;
    let t120885 = F::cast_from(2.0_f64) * t91669 * t33222;
    let t120887 = F::cast_from(3.0_f64) * t83886 * t33358;
    let t120888 = t24987 * t8641;
    let t120891 = F::cast_from(3.0_f64) * t22574 * t24432 * t120705;
    let t120892 = t7685 * t31295;
    (t120881, t120885, t120887, t120888, t120891, t120892)
}
