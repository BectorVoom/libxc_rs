//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 991/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk991<F: Float>(t22085: F, t22112: F, t225: F, t68: F, t484: F, t1177: F, t21749: F, t1196: F, t20217: F, t974: F, t11848: F, t20234: F, t11759: F, t21745: F, t3440: F, t11649: F, t1174: F, t1726: F, t18310: F, t18312: F, t18314: F, t18321: F, t18325: F, t18327: F, t18330: F, t18333: F, t22012: F, t22015: F, t488: F, t4889: F, t6178: F, t6184: F, t6188: F) -> (F, F, F, F, F, F, F) {
    let t22113 = t22085 + t22112;
    let t22114 = t22113 * t225;
    let t22115 = t22114 * t68;
    let t22116 = t22115 * t484;
    let t22119 = t1177 * t21749;
    let t22128 = t1196 * t20217;
    let t22129 = t974 * t22128;
    let t22132 = t11848 * t20234;
    let t22133 = t974 * t22132;
    let t22136 = t11759 * t20234;
    let t22137 = t974 * t22136;
    let t22149 = t3440 * t21745;
    let t22152 = -7.0 / 648.0 * t1174 * t22012 - t22015 * t488 / 192.0 + t22116 * t488 / 3072.0 - t1174 * t22119 / 48.0 + t11649 - t4889 * t6178 / 27.0 + t4889 * t6184 / 36.0 + t4889 * t6188 / 18.0 - t1174 * t22129 / 288.0 - t1174 * t22133 / 48.0 + t1174 * t22137 / 36.0 + t18310 / 1536.0 - t18312 / 144.0 + 19.0 / 864.0 * t18314 - t18325 / 144.0 + t18327 / 54.0 - t18330 / 288.0 + t18333 / 216.0 - 11.0 / 108.0 * t18321 * t1726 + t1174 * t22149 / 72.0;
    (t22113, t22114, t22115, t22129, t22133, t22137, t22152)
}
