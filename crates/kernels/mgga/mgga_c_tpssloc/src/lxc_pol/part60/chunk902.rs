//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 902/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk902<F: Float>(t115009: F, t115027: F, t121782: F, t126989: F, t128086: F, t128097: F, t128110: F, t1649: F, t1877: F, t1914: F, t23788: F, t24191: F, t2522: F, t26563: F, t26744: F, t26756: F, t28771: F, t28774: F, t28789: F, t28792: F, t28795: F, t31434: F, t33065: F, t33466: F, t33531: F, t5966: F, t7114: F, t7649: F, t7656: F, t8566: F, t89953: F, t92319: F) -> (F,) {
    let t128278 = -3.0 * t24191 * t126989 - 3.0 * t26756 * t89953 * t128110 + t1877 * t33466 * t1649 - t1877 * t31434 * t28795 / 2.0 - t1877 * t7114 * t5966 * t1914 / 2.0 + 3.0 * t2522 * t33466 * t7649 - t1877 * t26744 * t33065 - t1877 * t121782 * t7656 - 3.0 * t92319 * t33531 - 3.0 * t26563 * t23788 * t128097 - 3.0 / 2.0 * t24191 * t23788 * t128086 - t1877 * t31434 * t28792 + t1877 * t115027 * t28789 - 3.0 * t115009 * t28771 + 3.0 * t2522 * t8566 * t28774;
    (t128278,)
}
