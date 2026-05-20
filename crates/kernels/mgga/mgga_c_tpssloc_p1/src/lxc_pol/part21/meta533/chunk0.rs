//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2192/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2192<F: Float>(t1049: F, t5866: F, t1060: F, t1022: F, t11066: F, t5928: F, t3201: F, t4649: F, t1629: F, t11060: F, t4684: F, t5936: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18099 = t1049 * t5866;
    let t18100 = t18099 * t1060;
    let t18103 = t11066 * t1022;
    let t18104 = t5928 * t18103;
    let t18107 = t3201 * t4649;
    let t18108 = t1629 * t18107;
    let t18111 = t11060 * t1022;
    let t18112 = t5928 * t18111;
    let t18117 = t5936 * t4684;
    (t18099, t18100, t18103, t18104, t18107, t18108, t18111, t18112, t18117)
}
