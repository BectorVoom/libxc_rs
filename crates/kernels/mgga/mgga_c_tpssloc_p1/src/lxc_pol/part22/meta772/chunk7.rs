//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2639/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2639<F: Float>(t11881: F, t11883: F, t11904: F, t1215: F, t1235: F, t1244: F, t1246: F, t15245: F, t1755: F, t18940: F, t19146: F, t19201: F, t22243: F, t22348: F, t22365: F, t22389: F, t23508: F, t3610: F, t3612: F, t44785: F, t475: F, t4964: F, t5068: F, t5073: F, t5076: F, t52435: F, t6263: F, t6265: F, t73663: F) -> F {
    let t73844 = -t1215 * t22348 * t23508 * t44785 * t475 + t1235 * t1244 * t1246 * t22243 + F::new(6.0) * t1755 * t18940 * t3610 * t3612 + F::new(6.0) * t11881 * t11883 * t73663 + F::new(6.0) * t22389 * t3610 * t5068 + F::new(6.0) * t11904 * t22365 - F::new(3.0) * t15245 * t19146 + F::new(3.0) * t19201 * t5073 + F::new(3.0) * t19201 * t5076 + F::new(3.0) * t4964 * t6265 - F::new(3.0) * t52435 * t6263;
    t73844
}
