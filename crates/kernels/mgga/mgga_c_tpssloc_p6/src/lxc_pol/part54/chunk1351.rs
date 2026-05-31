//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1351/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1351<F: Float>(t12524: F, t33193: F, t4072: F, t576: F, t1395: F, t1458: F, t26135: F, t7230: F, t7015: F, t94170: F, t24465: F, t26550: F) -> (F, F, F, F, F, F) {
    let t120818 = F::cast_from(27.0_f64) * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t120865 = F::cast_from(0.135e2_f64) * t7230 * t26135;
    let t120867 = F::cast_from(27.0_f64) * t94170 * t7015;
    let t120869 = F::cast_from(27.0_f64) * t24465 * t26550;
    (t120818, t120833, t120849, t120865, t120867, t120869)
}
