//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1092/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1092<F: Float>(t1338: F, t5318: F, t1352: F, t12259: F, t1825: F, t3866: F, t5310: F, t1307: F, t5187: F, t3870: F, t820: F, t1799: F, t3719: F, t3799: F, t5289: F, t11984: F, t15876: F, t15878: F, t15880: F, t15887: F, t15888: F, t15889: F, t15891: F, t15894: F, t15896: F, t15898: F, t15910: F, t9457: F, t9476: F, t9484: F, t9780: F) -> (F, F, F, F, F, F, F) {
    let t16132 = t1338 * t5318;
    let t16133 = t16132 * t1352;
    let t16136 = t12259 * t1825;
    let t16147 = 35.0 / 576.0 * t3866 * t5310;
    let t16148 = t5187 * t1307;
    let t16150 = t3870 * t820 * t16148;
    let t16153 = t1799 * t3719;
    let t16155 = t3870 * t820 * t16153;
    let t16159 = 7.0 / 2304.0 * t3799 * t5289;
    let t16160 = -t9457 + t9476 + t9484 + t15876 - t15878 + t15880 - t15887 - t15888 - t15889 - t15891 - t15894 - t15896 - t11984 - t15898 + t9780 + t15910;
    (t16133, t16136, t16147, t16150, t16155, t16159, t16160)
}
