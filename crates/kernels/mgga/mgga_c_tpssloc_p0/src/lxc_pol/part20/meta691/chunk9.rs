//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2632/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2632<F: Float>(t11638: F, t11868: F, t11877: F, t11881: F, t1235: F, t1244: F, t1246: F, t14985: F, t14989: F, t15000: F, t15239: F, t1734: F, t1755: F, t3590: F, t3604: F, t3610: F, t3612: F, t3624: F, t3625: F, t470: F, t493: F, t5011: F, t5068: F, t5072: F, t5073: F, t5079: F, t52500: F, t53529: F) -> F {
    let t53538 = F::new(2.0) * t11638 * t1755 * t3610 * t3612 + t11868 * t1244 * t1246 * t1734 + F::new(3.0) * t1235 * t1244 * t1246 * t15239 + F::new(3.0) * t1244 * t1246 * t3590 * t5011 + F::new(18.0) * t11881 * t15000 * t5072 + F::new(6.0) * t14985 * t3610 * t5068 - F::new(3.0) * t14985 * t3624 * t5079 - F::new(3.0) * t3624 * t3625 * t52500 + t470 * t493 * t53529 + F::new(3.0) * t11877 * t5073 + F::new(6.0) * t14989 * t3604;
    t53538
}
