//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1122/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1122<F: Float>(t4899: F, t6138: F, t6144: F, t11588: F, t1887: F, t337: F, t5416: F, t3447: F, t4904: F, t51968: F, t3428: F, t6109: F, t1174: F, t6146: F, t698: F, t6140: F) -> (F, F, F, F, F, F, F, F, F) {
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64763 = t11588 * t6138;
    let t64779 = t11588 * t6144;
    let t64811 = t5416 * t337 * t1887;
    let t64821 = t3447 * t51968 * t4904;
    let t64878 = t6109 * t3428;
    let t64881 = t1174 * t698 * t6146;
    let t64885 = t1174 * t698 * t6140;
    (t64644, t64648, t64763, t64779, t64811, t64821, t64878, t64881, t64885)
}
