//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2171/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2171<F: Float>(t23788: F, t86797: F, t16596: F, t83555: F, t1081: F, t4303: F, t28: F, t40772: F, t86717: F, t1877: F, t22959: F, t23781: F, t23807: F, t23810: F, t23813: F, t25013: F, t2522: F, t25354: F, t25358: F, t25372: F, t25892: F, t25898: F, t25905: F, t4314: F, t6666: F, t6670: F, t6841: F, t7541: F, t81483: F, t86740: F, t86775: F, t86835: F, t87975: F) -> F {
    let t89928 = t23788 * t86797;
    let t89931 = t83555 * t16596;
    let t89941 = t1081 * t4303;
    let t89953 = t40772 * t28;
    let t89954 = t89953 * t86717;
    let t89957 = F::new(3.0) * t4314 * t7541 * t23781 - t1877 * t25358 * t23813 / F::new(2.0) - t86775 - F::new(6.0) * t25013 * t89928 - F::new(3.0) * t22959 * t89931 + F::new(3.0) * t2522 * t6666 * t25905 + t1877 * t87975 * t23807 + t1877 * t25354 * t1081 - t1877 * t6670 * t89941 - t1877 * t25358 * t23810 + F::new(3.0) * t2522 * t25354 * t6841 - F::new(3.0) * t81483 * t25898 - t86835 + F::new(6.0) * t86740 * t25892 - F::new(3.0) * t25372 * t89954;
    t89957
}
