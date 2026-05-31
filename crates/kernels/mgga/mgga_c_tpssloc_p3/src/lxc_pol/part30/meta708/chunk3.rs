//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2339/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339<F: Float>(t1858: F, t7758: F, t2029: F, t6470: F, t1851: F, t7774: F, t100867: F, t1396: F, t1852: F, t26555: F, t28904: F, t3: F, t5381: F, t580: F, t6483: F, t7003: F, t7759: F, t86579: F, t91813: F, t91816: F, t91818: F, t91824: F) -> F {
    let t100949 = t7758 * t1858;
    let t100952 = t6470 * t2029;
    let t100960 = t1851 * t7774;
    let t100962 = t100867 * t3 * t580 + t1396 * t28904 + F::cast_from(2.0_f64) * t1852 * t26555 + F::cast_from(2.0_f64) * t5381 * t7759 + t6483 * t7003 + F::cast_from(2.0_f64) * t100949 + t100952 + F::cast_from(2.0_f64) * t100960 + t86579 + t91813 + t91816 + t91818 + t91824;
    t100962
}
