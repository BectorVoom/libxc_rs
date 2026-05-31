//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1061/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1061<F: Float>(t13644: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10542: F, t10545: F, t10556: F, t10558: F, t10560: F, t10562: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F, t13675: F, t13679: F, t13692: F) -> F {
    let t13709 = F::cast_from(0.11038e0_f64) * t13644;
    let t13712 = F::cast_from(0.20128333333333333334e0_f64) * t13602;
    let t13714 = -F::cast_from(0.258925e1_f64) * t13630 - F::cast_from(0.1294625e1_f64) * t13632 + F::cast_from(0.19419375e1_f64) * t13635 - F::cast_from(0.412621875e-1_f64) * t13638 + F::cast_from(0.258925e1_f64) * t13640 - F::cast_from(0.91983333333333333334e-1_f64) * t13642 + t13709 - F::cast_from(0.82785e-1_f64) * t13647 - F::cast_from(0.13418888888888888889e0_f64) * t13598 + t13712 - F::cast_from(0.301925e0_f64) * t13613;
    let t13716 = -F::cast_from(0.5519e-1_f64) * t13530 - F::cast_from(0.27595e-1_f64) * t13534 - F::cast_from(0.36793333333333333333e-1_f64) * t13539 + F::cast_from(0.33114e0_f64) * t13544 + F::cast_from(0.16557e0_f64) * t13548 - t13675 + F::cast_from(0.36793333333333333334e-1_f64) * t13552 + F::cast_from(0.16557e0_f64) * t13557 - F::cast_from(0.49671e0_f64) * t13561 + t13679 + t13692 - t10542 - t10545 + F::cast_from(0.16504875e0_f64) * t13616 - F::cast_from(0.11038e0_f64) * t10300 - F::cast_from(0.26837777777777777778e0_f64) * t10556 + F::cast_from(0.67094444444444444447e-1_f64) * t10558 - F::cast_from(0.20128333333333333334e0_f64) * t10560 + F::cast_from(0.10064166666666666667e0_f64) * t10562 + F::cast_from(0.16504875e0_f64) * t13624 + F::cast_from(0.82524375e-1_f64) * t13626 + t13714;
    t13716
}
