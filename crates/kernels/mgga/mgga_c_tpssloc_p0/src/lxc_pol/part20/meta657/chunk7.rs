//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2435/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435<F: Float>(t49818: F, t10962: F, t4630: F, t13961: F, t3114: F, t10403: F, t10863: F, t14126: F, t14213: F, t14489: F, t14491: F, t17732: F, t3070: F, t3071: F, t3109: F, t42508: F, t43358: F, t4575: F, t4636: F, t49799: F, t49801: F, t49808: F, t49810: F, t884: F) -> F {
    let t49819 = t49818 / F::new(4608.0);
    let t49820 = t10962 * t4630;
    let t49822 = t3114 * t13961;
    let t49824 = t10403 * t3071 * t17732 * t14213 / F::new(384.0) + t3070 * t3071 * t14489 * t884 / F::new(1536.0) + t49799 / F::new(2304.0) + F::new(5.0) / F::new(3456.0) * t49801 + t42508 * t14126 / F::new(288.0) + F::new(19.0) / F::new(864.0) * t43358 * t4575 - t49808 / F::new(2304.0) + t49810 / F::new(2304.0) - t10863 * t4636 / F::new(144.0) - t3109 * t14491 / F::new(192.0) - t49819 + t49820 / F::new(1536.0) + t49822 / F::new(768.0);
    t49824
}
