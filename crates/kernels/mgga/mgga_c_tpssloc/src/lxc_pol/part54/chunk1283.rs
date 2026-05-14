//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1283/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1283<F: Float>(t24465: F, t26542: F, t26545: F, t16524: F, t31817: F, t12524: F, t33659: F, t31814: F, t2039: F, t26135: F, t3941: F, t20173: F, t33656: F, t1873: F, t27170: F, t6534: F, t7801: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122806 = 27.0 * t24465 * t26542;
    let t122808 = 27.0 * t24465 * t26545;
    let t122817 = 27.0 * t16524 * t31817;
    let t122824 = 27.0 * t12524 * t33659;
    let t122826 = 27.0 * t16524 * t31814;
    let t122829 = 27.0 * t3941 * t2039 * t26135;
    let t122831 = 27.0 * t20173 * t33656;
    let t122834 = 27.0 * t3941 * t27170 * t1873;
    let t122837 = 27.0 * t3941 * t7801 * t6534;
    (t122806, t122808, t122817, t122824, t122826, t122829, t122831, t122834, t122837)
}
