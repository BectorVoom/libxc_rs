//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2207/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2207<F: Float>(t1877: F, t1915: F, t22959: F, t25013: F, t25024: F, t2522: F, t25354: F, t25358: F, t25377: F, t25392: F, t28241: F, t28242: F, t28252: F, t28256: F, t28456: F, t4314: F, t46341: F, t6666: F, t7475: F, t7541: F, t81539: F, t97950: F, t97953: F, t97956: F, t97972: F, t97985: F) -> F {
    let t97989 = F::new(6.0) * t25013 * t97950 - F::new(3.0) * t22959 * t97953 - F::new(3.0) * t25013 * t97956 + F::new(3.0) * t2522 * t7541 * t25024 - t1877 * t25358 * t25392 + F::new(3.0) * t4314 * t6666 * t28241 + F::new(3.0) * t2522 * t25354 * t7475 + t97972 + t1877 * t81539 * t28456 - t1877 * t25358 * t25377 + F::new(3.0) / F::new(2.0) * t2522 * t6666 * t28256 + F::new(3.0) * t46341 * t28242 + F::new(3.0) * t2522 * t6666 * t28252 + F::new(3.0) * t4314 * t1915 * t97985;
    t97989
}
