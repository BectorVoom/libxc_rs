//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1452/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1452<F: Float>(t16524: F, t31817: F, t12524: F, t33659: F, t31814: F, t2039: F, t26135: F, t3941: F, t20173: F, t33656: F, t1873: F, t27170: F) -> (F, F, F, F, F, F) {
    let t122817 = F::cast_from(27.0_f64) * t16524 * t31817;
    let t122824 = F::cast_from(27.0_f64) * t12524 * t33659;
    let t122826 = F::cast_from(27.0_f64) * t16524 * t31814;
    let t122829 = F::cast_from(27.0_f64) * t3941 * t2039 * t26135;
    let t122831 = F::cast_from(27.0_f64) * t20173 * t33656;
    let t122834 = F::cast_from(27.0_f64) * t3941 * t27170 * t1873;
    (t122817, t122824, t122826, t122829, t122831, t122834)
}
