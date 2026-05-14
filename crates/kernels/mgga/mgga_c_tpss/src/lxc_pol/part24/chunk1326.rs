//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1326/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1326<F: Float>(t1482: F, t19949: F, t5017: F, t5640: F, t1483: F, t15131: F, t15151: F, t15191: F, t15200: F, t18150: F, t18156: F, t18171: F, t19889: F, t19892: F, t19901: F, t19913: F, t19919: F, t19932: F, t19933: F, t19939: F, t19942: F, t19946: F, t21411: F, t21414: F, t3994: F, t4016: F, t4017: F, t5036: F, t5623: F, t5626: F, t5631: F, t5632: F, t5642: F, t61285: F, t61522: F, t61567: F, t6174: F, t6175: F, t64515: F, t64590: F, t64613: F, t70560: F, t990: F) -> (F,) {
    let t70601 = t19949 * t1482;
    let t70645 = t5640 * t5017;
    let t70651 = -t5626 * t15200 + 4.0 * t64515 * t6175 + 4.0 * t18156 * t70601 * t5642 - 4.0 * t18171 * t19932 * t15151 - 12.0 * t5631 * t18150 * t6174 * t4016 + 4.0 * t19892 * t3994 + 2.0 * t5631 * t5632 * t5623 * t5036 - 4.0 * t64613 * t19933 + 2.0 * t64613 * t19942 - 2.0 * t64590 * t1483 + 4.0 * t5631 * t5632 * t19889 * t1482 + 4.0 * t19901 * t19946 + 4.0 * t19901 * t19919 - t61285 * t70560 * t15191 - 6.0 * t5626 * t15131 - 2.0 * t19892 * t4017 + 4.0 * t18156 * t19913 * t19939 - 12.0 * t5631 * t18150 * t21414 * t990 - 6.0 * t61522 * t70645 * t5642 + 4.0 * t61567 * t21411;
    (t70651,)
}
