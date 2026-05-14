//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 867/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk867<F: Float>(t135: F, t6146: F, t1174: F, t6140: F, t4889: F, t4916: F, t3403: F, t6084: F, t11285: F, t6068: F, t3359: F, t6052: F, t11352: F, t6036: F, t1098: F, t5983: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18529 = t135 * t6146;
    let t18530 = t1174 * t18529;
    let t18532 = t135 * t6140;
    let t18533 = t1174 * t18532;
    let t18536 = t4889 * t4916;
    let t18615 = t6084 * t3403;
    let t18622 = t6068 * t11285;
    let t18643 = t6052 * t3359;
    let t18650 = t6036 * t11352;
    let t18686 = t5983 * t1098;
    (t18529, t18530, t18532, t18533, t18536, t18615, t18622, t18643, t18650, t18686)
}
