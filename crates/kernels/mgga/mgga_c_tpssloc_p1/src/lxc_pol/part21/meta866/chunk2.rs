//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3162/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162<F: Float>(t11877: F, t11881: F, t11904: F, t11907: F, t1215: F, t1244: F, t1246: F, t19120: F, t19128: F, t19131: F, t19145: F, t19146: F, t19154: F, t19165: F, t19176: F, t19189: F, t19201: F, t3493: F, t3507: F, t3610: F, t3621: F, t44753: F, t44754: F, t45329: F, t5068: F, t5069: F, t52485: F, t6238: F, t6252: F, t6257: F) -> F {
    let t65408 = F::new(2.0) * t1215 * t1244 * t1246 * t19120 + t1244 * t1246 * t3493 * t6238 + F::new(14.0) * t3507 * t44753 * t44754 * t6252 + F::new(12.0) * t11881 * t19145 * t19165 + F::new(4.0) * t19128 * t3610 * t5068 + F::new(4.0) * t19189 * t3610 * t5068 + F::new(2.0) * t11877 * t6257 + F::new(4.0) * t11904 * t19176 - F::new(4.0) * t11907 * t19131 - F::new(2.0) * t11907 * t19146 + F::new(2.0) * t19154 * t45329 + t19201 * t3621 + F::new(8.0) * t5069 * t52485;
    t65408
}
