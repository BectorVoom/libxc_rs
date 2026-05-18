//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1081/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1081<F: Float>(t22132: F, t974: F, t11759: F, t20234: F, t21745: F, t3440: F, t11649: F, t1174: F, t1726: F, t18310: F, t18312: F, t18314: F, t18321: F, t18325: F, t18327: F, t18330: F, t18333: F, t22012: F, t22015: F, t22116: F, t22119: F, t22129: F, t488: F, t4889: F, t6178: F, t6184: F, t6188: F) -> (F, F, F) {
    let t22133 = t974 * t22132;
    let t22136 = t11759 * t20234;
    let t22137 = t974 * t22136;
    let t22149 = t3440 * t21745;
    let t22152 = -F::new(7.0) / F::new(648.0) * t1174 * t22012 - t22015 * t488 / F::new(192.0) + t22116 * t488 / F::new(3072.0) - t1174 * t22119 / F::new(48.0) + t11649 - t4889 * t6178 / F::new(27.0) + t4889 * t6184 / F::new(36.0) + t4889 * t6188 / F::new(18.0) - t1174 * t22129 / F::new(288.0) - t1174 * t22133 / F::new(48.0) + t1174 * t22137 / F::new(36.0) + t18310 / F::new(1536.0) - t18312 / F::new(144.0) + F::new(19.0) / F::new(864.0) * t18314 - t18325 / F::new(144.0) + t18327 / F::new(54.0) - t18330 / F::new(288.0) + t18333 / F::new(216.0) - F::new(11.0) / F::new(108.0) * t18321 * t1726 + t1174 * t22149 / F::new(72.0);
    (t22133, t22137, t22152)
}
