//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1090/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1090<F: Float>(t1362: F, t19815: F, t3799: F, t6417: F, t6422: F, t1307: F, t6330: F, t12351: F, t820: F, t1799: F, t5187: F, t3870: F, t1367: F, t19631: F, t16336: F, t1831: F) -> (F, F, F, F, F, F, F) {
    let t19904 = t19815 * t1362;
    let t19915 = t3799 * t6417;
    let t19917 = t3799 * t6422;
    let t19919 = t6330 * t1307;
    let t19921 = t12351 * t820 * t19919;
    let t19924 = t1799 * t5187;
    let t19926 = t3870 * t820 * t19924;
    let t19930 = t1367 * t820 * t19631;
    let t19933 = t16336 * t1831;
    (t19904, t19915, t19917, t19921, t19926, t19930, t19933)
}
