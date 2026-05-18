//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1218/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1218<F: Float>(t118413: F, t25927: F, t25365: F, t118466: F, t23788: F, t1081: F, t113111: F, t113131: F, t113135: F, t118399: F, t118406: F, t118439: F, t119691: F, t1877: F, t22959: F, t2522: F, t25372: F, t25898: F, t25901: F, t25905: F, t25921: F, t25930: F, t25934: F, t25938: F, t25945: F, t30753: F, t30757: F, t30770: F, t32886: F, t6848: F, t7649: F, t7656: F, t8370: F) -> F {
    let t119700 = t25927 * t118413;
    let t119713 = t25927 * t25365;
    let t119719 = t23788 * t118466;
    let t119733 = -F::new(3.0) / F::new(2.0) * t113131 * t25898 - t1877 * t30757 * t25945 / F::new(2.0) - F::new(3.0) * t118439 * t119691 - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25901 - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25905 + F::new(2.0) * t25372 * t119700 + F::new(3.0) / F::new(2.0) * t2522 * t30753 * t7649 + t1877 * t30770 * t25934 + t1877 * t32886 * t1081 / F::new(2.0) + t1877 * t30770 * t25930 + F::new(3.0) * t113135 * t119713 + t118406 - t1877 * t118399 * t6848 / F::new(2.0) - F::new(3.0) * t22959 * t119719 - F::new(3.0) / F::new(2.0) * t113131 * t25921 - t1877 * t30757 * t25930 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25938 - t1877 * t113111 * t7656 / F::new(2.0);
    t119733
}
