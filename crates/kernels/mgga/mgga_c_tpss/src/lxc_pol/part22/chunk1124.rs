//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1124/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1124<F: Float>(t18999: F, t509: F, t1270: F, t1845: F, t9909: F, t10456: F, t118: F, t1760: F, t1796: F, t1800: F, t1830: F, t1846: F, t18544: F, t18707: F, t18711: F, t18714: F, t18717: F, t18896: F, t18898: F, t18903: F, t18919: F, t18930: F, t2054: F, t2056: F, t2106: F, t3166: F, t485: F, t544: F, t5706: F, t5801: F, t5809: F, t5895: F, t5937: F, t624: F, t626: F, t646: F, t7798: F) -> (F, F, F, F) {
    let t19000 = t509 * t18999;
    let t19001 = t19000 * t1270;
    let t19005 = t1845 * t9909;
    let t19009 = -2.0 * t5801 * t2106 - 4.0 * t626 * t18707 + 6.0 * t1760 * t18711 + 2.0 * t1760 * t18714 + 3.0 * t1760 * t18717 + t18544 * t1846 - t118 * t18896 - 4.0 * t18898 * t646 + t18919 * t544 - t2054 * t1830 - 2.0 * t624 * t5895 - 2.0 * t7798 * t1800 - 4.0 * t10456 * t1800 - 4.0 * t2056 * t5809 - 4.0 * t626 * t18930 - t1796 * t3166 + t1760 * t19001 + 2.0 * t5706 * t5937 - t1760 * t19005 - 2.0 * t18903 * t485;
    (t19000, t19001, t19005, t19009)
}
