//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2196/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2196<F: Float>(t18124: F, t18164: F, t1055: F, t1052: F, t1066: F, t14529: F, t14545: F, t14552: F, t14555: F, t1635: F, t18053: F, t18057: F, t18059: F, t18062: F, t18065: F, t18071: F, t18074: F, t388: F, t4660: F, t4665: F) -> (F, F, F) {
    let t18165 = t18124 + t18164;
    let t18166 = t1055 * t18165;
    let t18168 = F::new(2.0) * t1052 * t18062 - F::new(6.0) * t1052 * t18071 - t1052 * t18166 - t1066 * t18074 - F::new(2.0) * t14529 * t1635 - F::new(2.0) * t14545 * t1635 - F::new(2.0) * t14552 * t1635 - F::new(2.0) * t14555 * t1635 + t18053 * t388 + t18057 * t388 + t18059 * t388 + F::new(2.0) * t18065 * t388 + F::new(4.0) * t4660 * t4665;
    (t18165, t18166, t18168)
}
