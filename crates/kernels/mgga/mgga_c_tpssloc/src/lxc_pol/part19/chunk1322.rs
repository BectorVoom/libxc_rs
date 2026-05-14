//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1322/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1322<F: Float>(t1176: F, t2402: F, t1174: F, t1179: F, t11529: F, t3460: F, t3456: F, t11516: F, t11547: F, t11569: F, t1177: F, t1178: F, t15395: F, t3440: F, t3447: F, t3455: F, t39097: F, t39103: F, t39110: F, t43711: F, t43732: F, t44602: F, t44608: F, t44621: F, t44622: F, t44628: F, t44631: F, t4900: F) -> (F,) {
    let t44633 = t2402 * t1176;
    let t44635 = t1174 * t44633 * t1179;
    let t44638 = t1174 * t11529 * t3460;
    let t44641 = t1174 * t11529 * t3456;
    let t44655 = 0.22222222222222222222e-2 * t44602 + 0.13333333333333333333e-1 * t3447 * t4900 * t43711 - 0.88888888888888888886e-2 * t3447 * t11569 * t44608 - 0.51851851851851851851e-2 * t3447 * t15395 * t43732 - 0.16666666666666666666e-2 * t1174 * t1177 * t3455 * t39103 + 0.28806584362139917695e-2 * t1174 * t44621 * t44622 * t39097 - 0.33333333333333333332e-2 * t44628 - 0.37037037037037037036e-3 * t44631 - 0.41152263374485596707e-3 * t44635 + 0.37037037037037037036e-3 * t44638 + 0.74074074074074074072e-3 * t44641 + 0.13333333333333333332e-1 * t1174 * t3440 * t11547 * t39097 - 0.66666666666666666664e-2 * t1174 * t1177 * t11516 * t39097 - 0.27777777777777777777e-3 * t1174 * t1177 * t1178 * t39110;
    (t44655,)
}
