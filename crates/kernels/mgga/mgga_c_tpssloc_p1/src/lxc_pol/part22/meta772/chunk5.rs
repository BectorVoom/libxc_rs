//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2637/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2637<F: Float>(t5052: F, t6224: F, t11881: F, t1215: F, t1244: F, t1246: F, t19165: F, t19201: F, t22340: F, t22348: F, t22358: F, t22364: F, t22368: F, t22386: F, t3610: F, t3624: F, t3625: F, t44698: F, t44701: F, t44753: F, t44754: F, t45326: F, t491: F, t5068: F, t5072: F, t5084: F, t6218: F, t72217: F) -> (F, F) {
    let t73720 = t5052 * t6224;
    let t73736 = -F::new(36.0) * t1215 * t22348 * t44698 * t44701 + F::new(14.0) * t1215 * t22348 * t44753 * t44754 + t1244 * t1246 * t491 * t72217 + F::new(3.0) * t1244 * t1246 * t5052 * t6218 + F::new(18.0) * t11881 * t19165 * t22364 + F::new(6.0) * t22340 * t3610 * t5068 + F::new(6.0) * t22368 * t3610 * t5072 + F::new(2.0) * t22386 * t3610 * t5068 - F::new(3.0) * t3624 * t3625 * t73720 + F::new(3.0) * t19201 * t5084 + F::new(6.0) * t22358 * t45326;
    (t73720, t73736)
}
