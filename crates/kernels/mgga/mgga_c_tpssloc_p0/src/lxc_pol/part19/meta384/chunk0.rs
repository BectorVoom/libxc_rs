//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1435/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1435<F: Float>(t1128: F, t11455: F, t3324: F, t3356: F, t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F) -> (F, F, F) {
    let t44295 = t11455 * t1128;
    let t44300 = t3324 * t3356;
    let t44314 = -F::cast_from(0.3044148148148148148e-1_f64) * t43748 - F::cast_from(0.25367901234567901233e-1_f64) * t43750 + F::cast_from(0.45662222222222222221e-1_f64) * t43780 + F::cast_from(0.9132444444444444444e-1_f64) * t43782 + F::cast_from(0.9132444444444444444e-1_f64) * t43784 - F::cast_from(0.13698666666666666667e0_f64) * t43786 - F::cast_from(0.22831111111111111111e-1_f64) * t43788 + F::cast_from(0.2283111111111111111e0_f64) * t43794 - F::cast_from(0.41095999999999999999e0_f64) * t43798 + F::new(0.41096e0) * t43802 + F::cast_from(0.17123333333333333333e-1_f64) * t43806;
    (t44295, t44300, t44314)
}
