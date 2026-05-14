//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1232/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1232<F: Float>(t13880: F, t13940: F, t1656: F, t1838: F, t1842: F, t18483: F, t18490: F, t18496: F, t18950: F, t18967: F, t19540: F, t19554: F, t20154: F, t20190: F, t21074: F, t21823: F, t21831: F, t21852: F, t4516: F, t51545: F, t5433: F, t5737: F, t5739: F, t5740: F, t5921: F, t5925: F, t60649: F, t60653: F, t62508: F, t6424: F, t6425: F, t65667: F, t66970: F, t69452: F, t69458: F, t69681: F, t69699: F, t69727: F, t69730: F, t69734: F, t69741: F) -> (F,) {
    let t71715 = 2.0 * t18483 * t21831 - 12.0 * t5739 * t18490 * t6424 * t4516 + 2.0 * t18950 * t5433 - 2.0 * t18496 * t18967 * t69699 + 6.0 * t60653 * t18967 * t69681 - 4.0 * t60649 * t21823 - 4.0 * t18496 * t62508 * t21074 - t5737 * t21852 - t69452 * t1842 + 4.0 * t5739 * t5740 * t20154 * t1656 + 4.0 * t65667 * t6425 + 2.0 * t5739 * t5740 * t1838 * t13940 + 2.0 * t69458 * t5925 - 2.0 * t19540 * t20190 * t51545 + 2.0 * t19540 * t66970 * t19554 - 2.0 * t18496 * t18967 * t69741 - 6.0 * t5921 * t13880 + t19540 * t18967 * t69727 - 2.0 * t18496 * t18967 * t69730 + 2.0 * t19540 * t18967 * t69734;
    (t71715,)
}
