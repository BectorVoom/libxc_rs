//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 953/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk953<F: Float>(t14676: F, t14677: F, t14685: F, t14987: F, t14988: F, t15532: F, t15609: F, t15610: F, t15611: F, t15615: F, t15619: F, t15620: F, t15909: F, t68354: F, t70735: F, t79971: F, t79976: F, t79980: F, t79988: F, t79993: F, t8: F, t80002: F, t80014: F, t80022: F, t80028: F, t80034: F, t80052: F, t80056: F, t80066: F, t80071: F, t80081: F, t80084: F, t80098: F, t80109: F, t80113: F, t80128: F, t80135: F, t80139: F, t80146: F, t80147: F, t80155: F, t80160: F, t80170: F, t80176: F, t80182: F, t80186: F, t80191: F, t80197: F, t80204: F, t80210: F, t80221: F, t80223: F, t80231: F, t80242: F, t80247: F, t80248: F, t80253: F, t80257: F, t80263: F, t80268: F, t80275: F, t80284: F, t80288: F, t80300: F, t80308: F, t80318: F, t80327: F, t80333: F, t80346: F, t80376: F, t80386: F, t80388: F, t80401: F, t80509: F, t80512: F, t80521: F, t80527: F, t80528: F, t80534: F, t80538: F) -> (F,) {
    let t80546 = t8 * (t80538 + t80534 + t80528 + t80527 + t80521 + t80512 + t80509 + t80401 + t80388 + t80386 + t80376 + t80346 + t80333 + t80327 + t80318 + t80308 + t80300 + t80288 + t80284 + t80275 + t80268 + t80263 + t80257 + t80253 + t80247 + t80248 + t80242 + t80231 + t80221 + t80223 + t80210 + t80204 + t80197 + t80191 + t80186 + t80182 + t80176 + t80170 + t80160 + t80155 + t80147 + t80146 + t80139 + t80135 + t80128 + t80113 + t80109 + t80098 + t80084 + t80081 + t80071 + t80066 + t80056 + t80052 + t80034 + t80028 + t80022 + t80014 + t80002 + t79993 + t79988 + t79980 + t79976 + t79971) - t70735 - t68354 + t15532 - t14676 - t14677 - t14987 - t14988 - t15609 - t15610 - t15611 - t15909 + t14685 - t15615 - t15619 - t15620;
    (t80546,)
}
