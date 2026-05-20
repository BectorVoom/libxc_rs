//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1151/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1151<F: Float>(t12442: F, t225: F, t12036: F, t12016: F, t12440: F, t3911: F, t12021: F, t12027: F, t12030: F, t12033: F, t12437: F, t12438: F, t12444: F, t1375: F, t1385: F, t1386: F, t3758: F, t3887: F, t3888: F, t3889: F, t3912: F) -> F {
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    let t39919 = t12440 * t225;
    let t39922 = t3911 * t3911;
    let t39932 = -F::new(36.0) * t12021 * t1375 * t3888 * t3911 + F::new(8.0) * t12437 * t1375 * t1385 * t3887 + F::new(6.0) * t1375 * t3887 * t39922 + F::new(24.0) * t12027 * t3758 - F::new(6.0) * t12030 * t3912 - F::new(6.0) * t12033 * t3912 - F::new(4.0) * t12438 * t3758 + F::new(24.0) * t12444 * t3889 - F::new(12.0) * t12444 * t3912 - F::new(4.0) * t1386 * t39910 - F::new(12.0) * t1386 * t39913 - F::new(12.0) * t1386 * t39916 - F::new(4.0) * t1386 * t39919;
    t39932
}
