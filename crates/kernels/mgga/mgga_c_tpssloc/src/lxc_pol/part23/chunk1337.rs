//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1337/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1337<F: Float>(t1174: F, t15569: F, t18321: F, t22119: F, t22154: F, t3555: F, t3577: F, t3578: F, t44805: F, t44817: F, t44938: F, t4889: F, t53490: F, t5975: F, t5979: F, t6178: F, t6192: F, t6219: F, t65884: F, t66622: F, t66668: F, t73142: F, t75836: F, t75847: F, t974: F) -> (F,) {
    let t79387 = -10.0 / 243.0 * t53490 - 19.0 / 216.0 * t66622 * t6192 - t3577 * t3578 * t6219 * t5979 / 768.0 - t3577 * t3578 * t6219 * t5975 / 384.0 - 154.0 / 243.0 * t73142 + 22.0 / 81.0 * t18321 * t6178 - t1174 * t974 * t3555 * t75847 / 48.0 + t1174 * t974 * t44938 * t75836 / 6.0 - 7.0 / 54.0 * t1174 * t974 * t44817 * t75836 + 35.0 / 972.0 * t1174 * t974 * t44805 * t75836 + t65884 * t6192 / 36.0 + t15569 * t22154 / 72.0 + 2.0 / 9.0 * t4889 * t22119 + t66668 / 216.0;
    (t79387,)
}
