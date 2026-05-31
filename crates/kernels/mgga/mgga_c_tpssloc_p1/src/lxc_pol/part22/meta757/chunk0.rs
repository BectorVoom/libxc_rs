//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2542/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542<F: Float>(t1102: F, t21785: F, t43889: F, t18746: F, t4756: F, t14813: F, t5999: F, t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F) -> (F, F, F, F) {
    let t71461 = t43889 * t21785 * t1102;
    let t71463 = t18746 * t4756;
    let t71465 = t14813 * t5999;
    let t71467 = -F::cast_from(0.60384999999999999999e0_f64) * t71183 - F::cast_from(0.60384999999999999999e0_f64) * t71187 + F::cast_from(0.82524375e-1_f64) * t71446 - F::cast_from(0.1294625e1_f64) * t71449 - F::cast_from(0.485484375e1_f64) * t71452 + F::cast_from(0.58258125e1_f64) * t71454 - F::cast_from(0.3883875e1_f64) * t71456 - F::cast_from(0.3883875e1_f64) * t71458 + F::cast_from(0.6189328125e-1_f64) * t71461 - F::cast_from(0.1237865625e0_f64) * t71463 + F::cast_from(0.247573125e0_f64) * t71465;
    (t71461, t71463, t71465, t71467)
}
