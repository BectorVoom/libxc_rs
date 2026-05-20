//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1990/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990<F: Float>(t100562: F, t16662: F, t16944: F, t16949: F, t17109: F, t1877: F, t2057: F, t24344: F, t2522: F, t25365: F, t25374: F, t26563: F, t26744: F, t4119: F, t4314: F, t5527: F, t5544: F, t5664: F, t67128: F, t67164: F, t7110: F, t7114: F, t7845: F, t84800: F, t93000: F, t98007: F, t98011: F, t98030: F) -> F {
    let t101892 = F::new(3.0) * t16662 * t2057 * t2522 + F::new(12.0) * t16944 * t2057 * t4314 + F::new(6.0) * t16949 * t2057 * t4314 - t17109 * t1877 * t7114 + F::new(4.0) * t1877 * t24344 * t98030 + F::new(4.0) * t1877 * t25374 * t93000 + F::new(2.0) * t1877 * t5664 * t84800 - F::new(6.0) * t2522 * t25365 * t26744 + F::new(6.0) * t2522 * t4119 * t7845 + F::new(3.0) * t2522 * t5544 * t7110 - F::new(6.0) * t2522 * t67164 * t7114 - F::new(6.0) * t2522 * t7114 * t98007 - F::new(3.0) * t2522 * t7114 * t98011 + F::new(6.0) * t4314 * t5527 * t7110 - F::new(6.0) * t4314 * t67128 * t7114 - F::new(12.0) * t100562 * t26563;
    t101892
}
