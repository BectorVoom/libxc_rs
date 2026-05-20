//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2280/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2280<F: Float>(t25927: F, t98111: F, t100682: F, t100689: F, t100692: F, t100696: F, t100705: F, t100708: F, t18196: F, t1877: F, t1915: F, t22959: F, t25013: F, t2522: F, t25358: F, t25372: F, t25898: F, t25945: F, t28: F, t28778: F, t28789: F, t6666: F, t6670: F, t6848: F, t81539: F, t86736: F, t98054: F, t98071: F, t99043: F) -> F {
    let t100713 = t25927 * t98111;
    let t100716 = -t1877 * t25358 * t25945 + t1877 * t81539 * t28789 + t1877 * t1915 * t18196 / F::new(2.0) - F::new(3.0) * t25372 * t100682 - t1877 * t98054 * t6848 / F::new(2.0) + F::new(2.0) * t25372 * t100689 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t100692 - t1877 * t6670 * t100696 + F::new(3.0) / F::new(2.0) * t2522 * t6666 * t28778 + t1877 * t99043 * t28 / F::new(2.0) - F::new(3.0) * t22959 * t100705 + F::new(6.0) * t25013 * t100708 - F::new(3.0) * t86736 * t25898 + F::new(6.0) * t22959 * t100713 + t98071;
    t100716
}
