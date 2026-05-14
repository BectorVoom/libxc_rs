//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 827/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk827<F: Float>(t74867: F, t74873: F, t74891: F, t74896: F, t74901: F, t74903: F, t74909: F, t74913: F, t74915: F, t74917: F, t74921: F, t15489: F, t16156: F, t71196: F, t71204: F, t74870: F, t74889: F, t74919: F) -> (F,) {
    let t77246 = 0.638468998399467591e-4 * t74867;
    let t77247 = 0.81823984962736025184e-1 * t74873;
    let t77249 = 0.85129199786595678799e-5 * t74891;
    let t77250 = 0.85129199786595678799e-5 * t74896;
    let t77251 = 0.85129199786595678799e-5 * t74901;
    let t77252 = 0.2553875993597870364e-4 * t74903;
    let t77253 = 0.2553875993597870364e-4 * t74909;
    let t77254 = 0.2553875993597870364e-4 * t74913;
    let t77255 = 0.2553875993597870364e-4 * t74915;
    let t77256 = 0.79828278012425390427e-1 * t74917;
    let t77258 = 0.10227998120342003148e-1 * t74921;
    let t77259 = t16156 * t15489;
    let t77260 = 0.19863479950205658386e-4 * t77259;
    let t77261 = -t77246 + t74870 + t77247 - 0.58171619854173713846e-5 * t74889 + t77249 + t77250 + t77251 - t77252 - t77253 - t77254 + t77255 - t77256 + t71196 + 0.24527028530061914063e-5 * t74919 + t77258 + t71204 - t77260;
    (t77261,)
}
