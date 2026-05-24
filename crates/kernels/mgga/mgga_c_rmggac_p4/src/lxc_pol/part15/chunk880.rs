//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 880/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk880<F: Float>(t39570: F, t8906: F, t623: F, t8619: F, t8622: F, t16504: F, t34975: F, t552: F, t8455: F, t2344: F, t40193: F, t1368: F, t16503: F, t3369: F, t8435: F) -> (F, F, F, F, F) {
    let t44786 = t39570 * t8906;
    let t44788 = t623 * t8619;
    let t44789 = t44788 * t8622;
    let t44793 = t34975 * t16504 * t552 * t8455;
    let t44795 = t40193 * t2344;
    let t44799 = t16503 * t3369 * t1368 * t8435;
    (t44786, t44789, t44793, t44795, t44799)
}
