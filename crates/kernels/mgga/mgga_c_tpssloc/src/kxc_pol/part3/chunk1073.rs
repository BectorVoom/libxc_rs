//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1073/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1073<F: Float>(t15466: F, t15512: F, t15558: F, t15601: F, t15648: F, t15684: F, t15726: F, t15768: F, t493: F, t1215: F, t5052: F, t1246: F, t11888: F, t11904: F, t11907: F, t11914: F, t1201: F, t1244: F, t1247: F, t15032: F, t15241: F, t15245: F, t15248: F, t15253: F, t15257: F, t15426: F, t15430: F, t1758: F, t3565: F, t3604: F, t3610: F, t3621: F, t3624: F, t3626: F, t470: F, t494: F, t5064: F, t5069: F, t5076: F, t5080: F, t5084: F, t5086: F) -> (F, F) {
    let t15771 = t15466 + t15512 + t15558 + t15601 + t15648 + t15684 + t15726 + t15768;
    let t15772 = t493 * t15771;
    let t15776 = t5052 * t1215;
    let t15777 = t15776 * t1246;
    let t15785 = 2.0 * t15032 * t1247 + t1244 * t15241 - 2.0 * t11907 * t5080 - t15245 * t3626 - 6.0 * t11888 * t15248 + 2.0 * t3604 * t5076 + 2.0 * t3610 * t15253 + t3565 * t1758 - 2.0 * t3624 * t15257 + t15426 * t494 + t11914 * t15430 + t470 * t15772 + 2.0 * t1201 * t5086 + 2.0 * t1244 * t15777 + t5064 * t3621 + 2.0 * t3604 * t5084 + 4.0 * t11904 * t5069;
    (t15771, t15785)
}
