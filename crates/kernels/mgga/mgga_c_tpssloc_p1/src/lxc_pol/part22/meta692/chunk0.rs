//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2272/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2272<F: Float>(t15285: F, t4889: F, t17686: F, t44505: F, t15363: F, t1174: F, t15281: F, t18549: F, t18554: F, t11570: F, t17635: F, t11583: F) -> (F, F, F, F, F, F, F) {
    let t65008 = t4889 * t15285;
    let t65018 = t44505 * t17686;
    let t65023 = t4889 * t15363;
    let t65035 = t1174 * t15281 * t18549;
    let t65041 = t1174 * t15281 * t18554;
    let t65056 = t11570 * t17635;
    let t65077 = t11583 * t17635;
    (t65008, t65018, t65023, t65035, t65041, t65056, t65077)
}
